use std::{
    io::ErrorKind,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Barrier, Weak,
    },
};

use async_std::{
    channel::{unbounded, Receiver, Sender},
    io::{timeout, ReadExt, WriteExt},
    net::TcpStream,
    task::{self, JoinHandle},
};
use futures_lite::{
    io::{split, ReadHalf, WriteHalf},
    Future,
};
use log::{debug, error, warn};
use packet::Packet;
use stopper::Stopper;

use crate::{
    connection::PacketEvent,
    packet::{
        client::handshake::Handshake, ClientConfiguration, ClientHandshake, ClientLogin,
        ClientPlay, ClientStatus, ServerPacket,
    },
};

use super::{Config, EventDispatcher};

enum ClientState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

impl From<u8> for ClientState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Handshake,
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Configuration,
            _ => Self::Play,
        }
    }
}

pub struct Client {
    pub socket_addr: SocketAddr,
    pub config_arc: Arc<Config>,
    disconnect_sender: Sender<SocketAddr>,
    handle_option: Option<(JoinHandle<()>, JoinHandle<()>)>,
    stopper: Stopper,
    pub compressed_atomic: Arc<AtomicBool>,
    pub packet_sender: Sender<ServerPacket>,
}

impl Client {
    pub fn new(
        stream: TcpStream,
        socket_addr: SocketAddr,
        config_arc: Arc<Config>,
        disconnect_sender: Sender<SocketAddr>,
        dispatcher: EventDispatcher,
    ) -> Arc<Self> {
        let barrier_arc = Arc::new(Barrier::new(3));
        let client_arc = Arc::new_cyclic(|client_weak: &Weak<Client>| {
            let (read_stream, write_stream) = split(stream);
            let stopper = Stopper::new();
            let compressed_atomic = Arc::new(AtomicBool::new(false));
            let (packet_sender, packet_receiver) = unbounded::<ServerPacket>();
            let handle_option = Some((
                task::spawn(Self::read_thread(
                    barrier_arc.clone(),
                    client_weak.clone(),
                    stopper.clone(),
                    read_stream,
                    socket_addr,
                    compressed_atomic.clone(),
                    config_arc.clone(),
                    dispatcher,
                )),
                task::spawn(Self::write_thread(
                    stopper.clone(),
                    packet_receiver,
                    write_stream,
                    socket_addr,
                    compressed_atomic.clone(),
                    config_arc.clone(),
                )),
            ));

            Self {
                socket_addr,
                config_arc,
                disconnect_sender,
                handle_option,
                stopper,
                compressed_atomic,
                packet_sender,
            }
        });

        barrier_arc.wait();
        client_arc
    }

    async fn read_thread(
        barrier_arc: Arc<Barrier>,
        client_weak: Weak<Client>,
        stopper: Stopper,
        mut read_stream: ReadHalf<TcpStream>,
        socket_addr: SocketAddr,
        compressed_atomic: Arc<AtomicBool>,
        config_arc: Arc<Config>,
        dispatcher: EventDispatcher,
    ) {
        barrier_arc.wait();

        let client_arc = client_weak.upgrade().unwrap();
        let mut client_state = ClientState::Handshake;
        let mut buffer: Vec<u8> = Vec::new();
        let mut tmp_buffer = vec![0; 1024];

        'main: loop {
            let length = match stopper
                .stop_future(timeout(
                    config_arc.timeout,
                    read_stream.read(&mut tmp_buffer),
                ))
                .await
            {
                Some(Ok(0)) | None => {
                    warn!("{socket_addr} : Connection closed");

                    client_arc.disconnect().await;
                    break 'main;
                }
                Some(Ok(length)) => length,
                Some(Err(error)) => {
                    if let ErrorKind::TimedOut = error.kind() {
                        warn!("{socket_addr} : Connection timed out");
                    } else {
                        warn!("{socket_addr} : An error occurred, connection closed");
                    }

                    client_arc.disconnect().await;
                    break 'main;
                }
            };

            buffer.extend(&tmp_buffer[..length]);

            // parse packets from buffer
            while let Ok(packet) =
                Packet::from_bytes(&mut buffer, compressed_atomic.load(Ordering::Relaxed))
            {
                match client_state {
                    ClientState::Handshake => {
                        let packet = match ClientHandshake::try_from(packet) {
                            Ok(packet) => packet,
                            Err(error) => {
                                error!("{socket_addr} : {error}");
                                continue;
                            }
                        };

                        debug!("{socket_addr} : {:?}", packet);

                        #[allow(irrefutable_let_patterns)]
                        if let ClientHandshake::Handshake(Handshake {
                            protocol_version,
                            next_state,
                            ..
                        }) = packet
                        {
                            if protocol_version == config_arc.version.protocol {
                                client_state = ClientState::from(next_state as u8);
                            } else {
                                warn!("{socket_addr} : Wrong protocol version, connection closed");

                                client_arc.disconnect().await;
                                break 'main;
                            }
                        }
                    }
                    ClientState::Status => {
                        let packet = match ClientStatus::try_from(packet) {
                            Ok(packet) => packet,
                            Err(error) => {
                                error!("{socket_addr} : {error}");
                                continue;
                            }
                        };

                        debug!("{socket_addr} : {:?}", packet);

                        if let Err(error) = dispatcher
                            .status_rwlock
                            .read()
                            .await
                            .dispatch(&PacketEvent::new(packet, client_arc.clone()))
                            .await
                        {
                            error!("{socket_addr} : {error}");
                        }
                    }
                    ClientState::Login => {
                        let packet = match ClientLogin::try_from(packet) {
                            Ok(packet) => packet,
                            Err(error) => {
                                error!("{socket_addr} : {error}");
                                continue;
                            }
                        };

                        debug!("{socket_addr} : {:?}", packet);

                        if let Err(error) = dispatcher
                            .login_rwlock
                            .read()
                            .await
                            .dispatch(&PacketEvent::new(packet, client_arc.clone()))
                            .await
                        {
                            error!("{socket_addr} : {error}");
                        }
                    }
                    ClientState::Configuration => {
                        let packet = match ClientConfiguration::try_from(packet) {
                            Ok(packet) => packet,
                            Err(error) => {
                                error!("{socket_addr} : {error}");
                                continue;
                            }
                        };

                        debug!("{socket_addr} : {:?}", packet);

                        if let Err(error) = dispatcher
                            .configuration_rwlock
                            .read()
                            .await
                            .dispatch(&PacketEvent::new(packet, client_arc.clone()))
                            .await
                        {
                            error!("{socket_addr} : {error}");
                        }
                    }
                    ClientState::Play => {
                        let packet = match ClientPlay::try_from(packet) {
                            Ok(packet) => packet,
                            Err(error) => {
                                error!("{socket_addr} : {error}");
                                continue;
                            }
                        };

                        debug!("{socket_addr} : {:?}", packet);

                        if let Err(error) = dispatcher
                            .play_rwlock
                            .read()
                            .await
                            .dispatch(&PacketEvent::new(packet, client_arc.clone()))
                            .await
                        {
                            error!("{socket_addr} : {error}");
                        }
                    }
                }
            }
        }
    }

    async fn write_thread(
        stopper: Stopper,
        packet_receiver: Receiver<ServerPacket>,
        mut write_stream: WriteHalf<TcpStream>,
        socket_addr: SocketAddr,
        compressed_atomic: Arc<AtomicBool>,
        config_arc: Arc<Config>,
    ) {
        while let Some(Ok(packet)) = stopper.stop_future(packet_receiver.recv()).await {
            Self::write_packet(
                &mut write_stream,
                packet,
                &socket_addr,
                &compressed_atomic,
                &config_arc,
            )
            .await;
        }

        // ensure all pending packets are sent
        while let Ok(packet) = packet_receiver.try_recv() {
            Self::write_packet(
                &mut write_stream,
                packet,
                &socket_addr,
                &compressed_atomic,
                &config_arc,
            )
            .await;
        }
    }

    async fn write_packet(
        write_stream: &mut WriteHalf<TcpStream>,
        packet: ServerPacket,
        socket_addr: &SocketAddr,
        compressed_atomic: &Arc<AtomicBool>,
        config_arc: &Arc<Config>,
    ) {
        match Packet::try_from(packet) {
            Ok(packet) => match packet.into_bytes(
                compressed_atomic.load(Ordering::Relaxed),
                config_arc.compression_threshold,
            ) {
                Ok(buffer) => {
                    if let Err(error) = write_stream.write_all(&buffer).await {
                        error!("{socket_addr} : Sending a packet : {error}");
                    }
                }
                Err(error) => {
                    error!("{socket_addr} : Error serializing a packet : {error}");
                }
            },
            Err(error) => error!("{socket_addr} : {error}"),
        };
    }

    pub async fn disconnect(&self) {
        self.stopper.stop();
        self.disconnect_sender.send(self.socket_addr).await.ok();
    }
}

pub(super) trait ClientStop {
    fn stop(self) -> impl Future<Output = ()> + Send;
}

impl ClientStop for Arc<Client> {
    async fn stop(self) {
        if let Some(mut client) = Arc::into_inner(self) {
            client.stopper.stop();

            if let Some(handle) = client.handle_option.take() {
                handle.0.await;
                handle.1.await;
            }
        }
    }
}

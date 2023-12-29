use std::{
    io::ErrorKind,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
};

use async_std::{
    channel::{unbounded, Sender},
    io::{timeout, ReadExt, WriteExt},
    net::TcpStream,
    sync::RwLock,
    task::{self, JoinHandle},
};
use epicenter::AsyncDispatcher;
use futures_lite::{io::split, Future};
use log::{debug, error, warn};
use packet::Packet;

use crate::{
    connection::PacketEvent,
    packet::{
        client::handshake::Handshake, ClientConfiguration, ClientHandshake, ClientLogin,
        ClientPlay, ClientStatus, ServerPacket,
    },
};

use super::Config;

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
    pub client_weak: Weak<Client>,
    pub socket_addr: SocketAddr,
    pub config_arc: Arc<Config>,
    handle: (JoinHandle<()>, JoinHandle<()>),
    pub running_atomic: Arc<AtomicBool>,
    pub compressed_atomic: Arc<AtomicBool>,
    pub packet_sender: Sender<ServerPacket>,
}

impl Client {
    pub fn new(
        stream: TcpStream,
        socket_addr: SocketAddr,
        config_arc: Arc<Config>,
        dispatcher_status_rwlock: Arc<RwLock<AsyncDispatcher>>,
        dispatcher_login_rwlock: Arc<RwLock<AsyncDispatcher>>,
        dispatcher_configuration_rwlock: Arc<RwLock<AsyncDispatcher>>,
        dispatcher_play_rwlock: Arc<RwLock<AsyncDispatcher>>,
    ) -> Arc<Self> {
        let (mut read_stream, mut write_stream) = split(stream);
        let running_atomic = Arc::new(AtomicBool::new(true));
        let compressed_atomic = Arc::new(AtomicBool::new(false));
        let (packet_sender, packet_receiver) = unbounded::<ServerPacket>();

        Arc::new_cyclic(|client_weak| {
            let handle = (
                task::spawn({
                    let client_weak = client_weak.clone();
                    let config_arc = config_arc.clone();
                    let running_atomic = running_atomic.clone();
                    let compressed_atomic = compressed_atomic.clone();

                    async move {
                        let mut client_state = ClientState::Handshake;
                        let mut buffer: Vec<u8> = Vec::new();
                        let mut tmp_buffer = vec![0; 1024];

                        'main: loop {
                            let length = match timeout(
                                config_arc.timeout,
                                read_stream.read(&mut tmp_buffer),
                            )
                            .await
                            {
                                Ok(length) => length,
                                Err(error) => {
                                    if let ErrorKind::TimedOut = error.kind() {
                                        warn!("{socket_addr} : Connection timed out");
                                    } else {
                                        warn!(
                                            "{socket_addr} : An error occurred, connection closed"
                                        );
                                    }

                                    running_atomic.store(false, Ordering::Relaxed);
                                    break 'main;
                                }
                            };

                            if length == 0 {
                                warn!("{socket_addr} : Connection closed");

                                running_atomic.store(false, Ordering::Relaxed);
                                break 'main;
                            }

                            buffer.extend(&tmp_buffer[..length]);

                            while let Ok(packet) = Packet::from_bytes(
                                &mut buffer,
                                compressed_atomic.load(Ordering::Relaxed),
                            ) {
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

                                                running_atomic.store(false, Ordering::Relaxed);
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

                                        if let Err(error) = dispatcher_status_rwlock
                                            .read()
                                            .await
                                            .dispatch(&PacketEvent::new(
                                                packet,
                                                client_weak.upgrade().unwrap(),
                                            ))
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

                                        if let Err(error) = dispatcher_login_rwlock
                                            .read()
                                            .await
                                            .dispatch(&PacketEvent::new(
                                                packet,
                                                client_weak.upgrade().unwrap(),
                                            ))
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

                                        if let Err(error) = dispatcher_configuration_rwlock
                                            .read()
                                            .await
                                            .dispatch(&PacketEvent::new(
                                                packet,
                                                client_weak.upgrade().unwrap(),
                                            ))
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

                                        if let Err(error) = dispatcher_play_rwlock
                                            .read()
                                            .await
                                            .dispatch(&PacketEvent::new(
                                                packet,
                                                client_weak.upgrade().unwrap(),
                                            ))
                                            .await
                                        {
                                            error!("{socket_addr} : {error}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }),
                task::spawn({
                    let config_arc = config_arc.clone();
                    let compressed_atomic = compressed_atomic.clone();

                    async move {
                        while let Ok(packet) = packet_receiver.recv().await {
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
                                        error!(
                                            "{socket_addr} : Error serializing a packet : {error}"
                                        );
                                    }
                                },
                                Err(error) => error!("{socket_addr} : {error}"),
                            };
                        }
                    }
                }),
            );

            Self {
                client_weak: client_weak.clone(),
                socket_addr,
                config_arc,
                handle,
                running_atomic,
                compressed_atomic,
                packet_sender,
            }
        })
    }
}

pub trait ClientDisconnect {
    fn disconnect(self) -> impl Future<Output = ()> + Send;
}

impl ClientDisconnect for Arc<Client> {
    async fn disconnect(self) {
        if let Some(client) = Arc::into_inner(self) {
            client.handle.0.cancel().await;
            client.handle.1.cancel().await;
        }
    }
}

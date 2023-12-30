use std::{
    io::ErrorKind,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Barrier, Weak,
    },
};

use anyhow::Result;
use async_std::{
    channel::Sender,
    io::{timeout, ReadExt, WriteExt},
    net::TcpStream,
    sync::{Mutex, MutexGuard},
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

use super::{AesDecryptor, AesEncryptor, Config, EventDispatcher, RsaEncryptor};

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

pub struct Client<T: Send + Sync + 'static> {
    pub socket_addr: SocketAddr,
    pub config_arc: Arc<Config>,
    pub encryptor_arc: Arc<RsaEncryptor>,
    data_option_mutex: Mutex<Option<T>>,
    wrong_protocol_version_atomic: Arc<AtomicBool>,
    disconnect_sender: Sender<SocketAddr>,
    handle_option: Option<JoinHandle<()>>,
    stopper: Stopper,
    compressed_atomic: Arc<AtomicBool>,
    aes_encryptor_option_mutex: Mutex<Option<AesEncryptor>>,
    aes_decryptor_option_mutex: Arc<Mutex<Option<AesDecryptor>>>,
    write_stream_mutex: Mutex<WriteHalf<TcpStream>>,
}

impl<T: Send + Sync + 'static> Client<T> {
    pub fn new(
        stream: TcpStream,
        socket_addr: SocketAddr,
        config_arc: Arc<Config>,
        encryptor_arc: Arc<RsaEncryptor>,
        disconnect_sender: Sender<SocketAddr>,
        dispatcher: EventDispatcher,
    ) -> Arc<Self> {
        let barrier_arc = Arc::new(Barrier::new(2));
        let client_arc = Arc::new_cyclic(|client_weak: &Weak<Client<T>>| {
            let (read_stream, write_stream) = split(stream);
            let wrong_protocol_version_atomic = Arc::new(AtomicBool::new(false));
            let stopper = Stopper::new();
            let aes_decryptor_option_mutex = Arc::new(Mutex::new(None));
            let compressed_atomic = Arc::new(AtomicBool::new(false));
            let handle_option = Some(task::spawn(Self::read_thread(
                barrier_arc.clone(),
                client_weak.clone(),
                wrong_protocol_version_atomic.clone(),
                stopper.clone(),
                read_stream,
                socket_addr,
                config_arc.clone(),
                aes_decryptor_option_mutex.clone(),
                compressed_atomic.clone(),
                dispatcher,
            )));

            Self {
                socket_addr,
                config_arc,
                encryptor_arc,
                data_option_mutex: Mutex::new(None),
                wrong_protocol_version_atomic,
                disconnect_sender,
                handle_option,
                stopper,
                compressed_atomic,
                aes_encryptor_option_mutex: Mutex::new(None),
                aes_decryptor_option_mutex,
                write_stream_mutex: Mutex::new(write_stream),
            }
        });

        barrier_arc.wait();
        client_arc
    }

    async fn read_thread(
        barrier_arc: Arc<Barrier>,
        client_weak: Weak<Client<T>>,
        wrong_protocol_version_atomic: Arc<AtomicBool>,
        stopper: Stopper,
        mut read_stream: ReadHalf<TcpStream>,
        socket_addr: SocketAddr,
        config_arc: Arc<Config>,
        aes_decryptor_option_mutex: Arc<Mutex<Option<AesDecryptor>>>,
        compressed_atomic: Arc<AtomicBool>,
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

            if let Some(aes_decryptor) = aes_decryptor_option_mutex.lock().await.as_mut() {
                aes_decryptor.decrypt(&mut tmp_buffer[..length]);
            }

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
                            client_state = ClientState::from(next_state as u8);

                            wrong_protocol_version_atomic.store(
                                protocol_version != config_arc.version.protocol,
                                Ordering::Relaxed,
                            );
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

                        if let ClientLogin::LoginAcknowledged(_) = packet {
                            client_state = ClientState::Configuration;
                        }

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

    pub async fn send_packet(&self, packet: ServerPacket) {
        match Packet::try_from(packet) {
            Ok(packet) => match packet.into_bytes(
                self.compressed_atomic.load(Ordering::Relaxed),
                self.config_arc.compression_threshold,
            ) {
                Ok(mut buffer) => {
                    if let Some(aes_encryptor) =
                        self.aes_encryptor_option_mutex.lock().await.as_mut()
                    {
                        aes_encryptor.encrypt(&mut buffer);
                    }

                    if let Err(error) = self
                        .write_stream_mutex
                        .lock()
                        .await
                        .write_all(&buffer)
                        .await
                    {
                        error!("{} : Sending a packet : {error}", self.socket_addr);
                    }
                }
                Err(error) => {
                    error!(
                        "{} : Error serializing a packet : {error}",
                        self.socket_addr
                    );
                }
            },
            Err(error) => error!("{} : {error}", self.socket_addr),
        };
    }

    pub fn wrong_protocol_version(&self) -> bool {
        self.wrong_protocol_version_atomic.load(Ordering::Relaxed)
    }

    pub async fn set_data(&self, data: T) {
        *self.data_option_mutex.lock().await = Some(data);
    }

    pub async fn data(&self) -> MutexGuard<Option<T>> {
        self.data_option_mutex.lock().await
    }

    pub async fn enable_encryption(&self, shared_secret: &Vec<u8>) -> Result<()> {
        let (aes_encryptor, aes_decryptor) = self.encryptor_arc.aes_from_secret(shared_secret)?;

        *self.aes_encryptor_option_mutex.lock().await = Some(aes_encryptor);
        *self.aes_decryptor_option_mutex.lock().await = Some(aes_decryptor);

        Ok(())
    }

    pub fn enable_compression(&self) {
        self.compressed_atomic.store(true, Ordering::Relaxed);
    }

    pub async fn disconnect(&self) {
        self.stopper.stop();
        self.disconnect_sender.send(self.socket_addr).await.ok();
    }
}

pub(super) trait ClientStop {
    fn stop(self) -> impl Future<Output = ()> + Send;
}

impl<T: Send + Sync + 'static> ClientStop for Arc<Client<T>> {
    async fn stop(self) {
        if let Some(mut client) = Arc::into_inner(self) {
            client.stopper.stop();

            if let Some(handle) = client.handle_option.take() {
                handle.await;
            }
        }
    }
}

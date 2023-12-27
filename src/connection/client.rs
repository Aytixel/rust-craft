use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use async_std::{
    io::ReadExt,
    net::TcpStream,
    task::{self, JoinHandle},
};
use futures_lite::io::{split, WriteHalf};
use log::{debug, error};
use packet::Packet;
use try_catch::catch;

use crate::packet::{
    client::{
        configuration::ClientConfiguration, handshake::Handshake, login::ClientLogin,
        play::ClientPlay, status::ClientStatus,
    },
    ClientHandshake,
};

use super::Config;

#[derive(Clone)]
#[repr(u8)]
pub enum ClientState {
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
    write_stream: WriteHalf<TcpStream>,
    handle: JoinHandle<()>,
    pub running_atomic: Arc<AtomicBool>,
}

impl Client {
    pub fn new(stream: TcpStream, socket_addr: SocketAddr, config_arc: Arc<Config>) -> Self {
        let (mut read_stream, write_stream) = split(stream);
        let running_atomic = Arc::new(AtomicBool::new(true));

        let handle = task::spawn({
            let running_atomic = running_atomic.clone();

            async move {
                let mut client_state = ClientState::Handshake;
                let mut compressed = false;
                let mut buffer: Vec<u8> = Vec::new();
                let mut tmp_buffer = vec![0; 1024];

                'main: while let Ok(length) = read_stream.read(&mut tmp_buffer).await {
                    if length == 0 {
                        break;
                    }

                    buffer.extend(&tmp_buffer[..length]);

                    while let Ok(packet) = Packet::from_bytes(&mut buffer, compressed) {
                        catch! {
                            try {
                                match client_state {
                                    ClientState::Handshake => {
                                        let packet = ClientHandshake::try_from(packet)?;

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
                                                error!("{socket_addr} : Wrong protocol version");

                                                running_atomic.store(false, Ordering::Relaxed);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    ClientState::Status => {
                                        let packet = ClientStatus::try_from(packet)?;

                                        debug!("{socket_addr} : {:?}", packet);
                                    },
                                    ClientState::Login => {
                                        let packet = ClientLogin::try_from(packet)?;

                                        debug!("{socket_addr} : {:?}", packet);
                                    },
                                    ClientState::Configuration => {
                                        let packet = ClientConfiguration::try_from(packet)?;

                                        debug!("{socket_addr} : {:?}", packet);
                                    },
                                    ClientState::Play => {
                                        let packet = ClientPlay::try_from(packet)?;

                                        debug!("{socket_addr} : {:?}", packet);
                                    },
                                }
                            }
                            catch error {
                                error!("{socket_addr} : {error}")
                            }
                        }

                        if !running_atomic.load(Ordering::Relaxed) {
                            break 'main;
                        }
                    }
                }
            }
        });

        Self {
            socket_addr,
            write_stream,
            handle,
            running_atomic,
        }
    }

    pub async fn disconnect(self) {
        self.handle.cancel().await;
    }
}

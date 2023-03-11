use std::io::{self, ErrorKind, Read};
use std::net::{Shutdown, SocketAddr, TcpStream};

use log::{debug, error};

use crate::data_type::Packet;
use crate::packet::client::HandshakePacket;

enum ClientState {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Client {
    socket: TcpStream,
    pub socket_addr: SocketAddr,
    state: ClientState,
    buffer: Vec<u8>,
    compressed: bool,
}

impl Client {
    pub fn new(socket: TcpStream, socket_addr: SocketAddr) -> io::Result<Self> {
        if let Err(e) = socket.set_nonblocking(true) {
            socket.shutdown(Shutdown::Both)?;

            return Err(e);
        }

        Ok(Self {
            socket,
            socket_addr,
            state: ClientState::Handshake,
            buffer: vec![],
            compressed: false,
        })
    }

    pub fn update(&mut self) -> io::Result<()> {
        let mut buffer = [0u8; 2048];
        let length = match self.socket.read(&mut buffer) {
            Ok(v) => v,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(()),
            Err(e) => return Err(e),
        };

        self.buffer.append(&mut buffer[0..length].to_vec());

        match Packet::try_from(&mut self.buffer, self.compressed) {
            Ok(packet) => {
                if let Err(e) = match self.state {
                    ClientState::Handshake => self.handshake(&packet),
                    ClientState::Status => self.status(&packet),
                    ClientState::Login => self.login(&packet),
                    ClientState::Play => todo!(),
                } {
                    error!("{e:?}");
                }
            }
            Err(_) => {}
        }

        Ok(())
    }

    fn handshake(&mut self, packet: &Packet) -> Result<(), &'static str> {
        if packet.id == 0 {
            let handshake_packet = HandshakePacket::try_from(packet.clone())?;

            debug!("{:?}", handshake_packet);

            if (handshake_packet.next_state == 1) {
                self.state = ClientState::Status;
            } else {
                self.state = ClientState::Login;
            }
        }

        Ok(())
    }

    fn status(&self, packet: &Packet) -> Result<(), &'static str> {
        Ok(())
    }

    fn login(&self, packet: &Packet) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn disconnect(&self) -> io::Result<()> {
        self.socket.shutdown(Shutdown::Both)
    }
}

use std::io::{self, ErrorKind, Read};
use std::net::{Shutdown, SocketAddr, TcpStream};

use log::debug;

use crate::data_type::packet::Packet;

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
            compressed: true,
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
                debug!("{:?}", packet);

                match self.state {
                    ClientState::Handshake => todo!(),
                    ClientState::Status => todo!(),
                    ClientState::Login => todo!(),
                    ClientState::Play => todo!(),
                }
            }
            Err(e) => debug!("{:?}", e),
        }

        Ok(())
    }

    pub fn disconnect(&self) -> io::Result<()> {
        self.socket.shutdown(Shutdown::Both)
    }
}

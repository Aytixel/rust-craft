use std::io::{self, ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};

use log::error;
use num::FromPrimitive;

use crate::data_type::Packet;
use crate::packet::handshake::HandshakePacket;
use crate::packet::status::{PingPacket, StatusPacket};
use crate::packet::{ServerHandshakePacketId, ServerStatusPacketId};

pub enum ClientState {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Client {
    socket: TcpStream,
    pub socket_addr: SocketAddr,
    pub state: ClientState,
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

        while let Ok(packet) = Packet::try_from(&mut self.buffer, self.compressed) {
            let response_packet = match self.state {
                ClientState::Handshake => self.handshake(&packet),
                ClientState::Status => self.status(&packet),
                ClientState::Login => self.login(&packet),
                ClientState::Play => todo!(),
            };

            match response_packet {
                Ok(Some(packet)) => {
                    self.socket.write_all(&packet.try_into(self.compressed)?)?;
                    self.socket.flush()?;
                }
                Ok(None) => {}
                Err(e) => error!("{e:?}"),
            }
        }

        Ok(())
    }

    fn handshake(&mut self, packet: &Packet) -> Result<Option<Packet>, &'static str> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerHandshakePacketId::Handshake => HandshakePacket::handle(&mut self.state, packet),
        }
    }

    fn status(&self, packet: &Packet) -> Result<Option<Packet>, &'static str> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerStatusPacketId::Status => StatusPacket::handle(),
            ServerStatusPacketId::Ping => PingPacket::handle(packet),
        }
    }

    fn login(&self, _packet: &Packet) -> Result<Option<Packet>, &'static str> {
        Ok(None)
    }

    pub fn disconnect(&self) -> io::Result<()> {
        self.socket.shutdown(Shutdown::Both)
    }
}

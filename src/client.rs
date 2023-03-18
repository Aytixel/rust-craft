use std::io::{self, ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::rc::Rc;

use log::error;
use num::FromPrimitive;
use uuid::Uuid;

use crate::data_type::Packet;
use crate::packet::handshake::HandshakePacket;
use crate::packet::login::{EncryptionPacket, LoginStartPacket};
use crate::packet::status::{PingPacket, StatusPacket};
use crate::packet::{ServerHandshakePacketId, ServerLoginPacketId, ServerStatusPacketId};
use crate::server::EncryptionData;

pub const COMPRESSION_THRESHOLD: i32 = 256;

pub enum ClientState {
    Handshake,
    Status,
    Login,
    Play,
}

#[derive(Default)]
pub struct PlayerData {
    pub username: String,
    pub uuid: Uuid,
}

pub struct Client {
    socket: TcpStream,
    pub socket_addr: SocketAddr,
    pub state: ClientState,
    buffer: Vec<u8>,
    pub compressed: bool,
    pub encryption_data: Rc<EncryptionData>,
    pub player_data: Rc<PlayerData>,
}

impl Client {
    pub fn new(
        socket: TcpStream,
        socket_addr: SocketAddr,
        encryption_data: Rc<EncryptionData>,
    ) -> io::Result<Self> {
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
            encryption_data,
            player_data: Rc::new(PlayerData::default()),
        })
    }

    pub fn update(&mut self) -> io::Result<()> {
        let mut buffer = [0u8; 2048];
        let length = match self.socket.read(&mut buffer) {
            Ok(v) => v,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(()),
            Err(e) => return Err(e),
        };

        self.buffer.append(&mut buffer[..length].to_vec());

        while let Ok(packet) = Packet::try_from(&mut self.buffer, self.compressed) {
            if let Err(e) = match self.state {
                ClientState::Handshake => self.handshake(&packet),
                ClientState::Status => self.status(&packet),
                ClientState::Login => self.login(&packet),
                ClientState::Play => todo!(),
            } {
                error!("{e}");
            }
        }

        Ok(())
    }

    pub fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        self.socket.write_all(&packet.try_into(self.compressed)?)?;
        self.socket.flush()?;

        Ok(())
    }

    fn handshake(&mut self, packet: &Packet) -> Result<(), &'static str> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerHandshakePacketId::Handshake => HandshakePacket::handle(&mut self.state, packet),
        }
    }

    fn status(&mut self, packet: &Packet) -> Result<(), &'static str> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerStatusPacketId::Status => StatusPacket::handle(self),
            ServerStatusPacketId::Ping => PingPacket::handle(self, packet),
        }
    }

    fn login(&mut self, packet: &Packet) -> Result<(), &'static str> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerLoginPacketId::LoginStart => LoginStartPacket::handle(self, packet),
            ServerLoginPacketId::Encryption => EncryptionPacket::handle(self, packet),
            ServerLoginPacketId::LoginPlugin => todo!(),
        }
    }

    pub fn disconnect(&self) -> io::Result<()> {
        self.socket.shutdown(Shutdown::Both)
    }
}

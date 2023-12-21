use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::rc::Rc;

use aes::cipher::AsyncStreamCipher;
use aes::Aes128;
use cfb8::Encryptor;
use log::{debug, error, info};
use num::FromPrimitive;
use uuid::Uuid;

use crate::data_type::Packet;
use crate::datapack::Datapack;
use crate::packet::handshake::HandshakePacket;
use crate::packet::login::{EncryptionPacket, LoginStartPacket};
use crate::packet::status::{PingPacket, StatusPacket};
use crate::packet::{ServerHandshakePacketId, ServerLoginPacketId, ServerStatusPacketId};
use crate::server::EncryptionData;
use crate::version_info::VersionInfo;

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

pub struct Aes {
    pub encrypt: Encryptor<Aes128>,
}

impl Aes {
    pub fn encrypt(&self, buffer: Vec<u8>) -> Vec<u8> {
        let mut buffer = buffer;

        self.encrypt.clone().encrypt(&mut buffer);

        buffer
    }
}

pub struct Client {
    socket: TcpStream,
    pub socket_addr: SocketAddr,
    pub state: ClientState,
    buffer: Vec<u8>,
    pub compressed: bool,
    pub encryption_data: Rc<EncryptionData>,
    pub aes: Option<Aes>,
    pub version_info: Rc<VersionInfo>,
    pub player_data: Rc<PlayerData>,
    datapack: Rc<Datapack>,
}

impl Client {
    pub fn new(
        socket: TcpStream,
        socket_addr: SocketAddr,
        encryption_data: Rc<EncryptionData>,
        version_info: Rc<VersionInfo>,
        datapack: Rc<Datapack>,
    ) -> Result<Self, String> {
        debug!("New tcp client: {socket_addr}");

        socket.set_nonblocking(true).map_err(|e| e.to_string())?;

        Ok(Self {
            socket,
            socket_addr,
            state: ClientState::Handshake,
            buffer: vec![],
            compressed: false,
            aes: None,
            encryption_data,
            version_info,
            player_data: Rc::new(PlayerData::default()),
            datapack,
        })
    }

    pub fn update(&mut self) -> Result<(), String> {
        let mut buffer = [0u8; 2048];
        let length = match self.socket.read(&mut buffer) {
            Ok(v) => v,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(()),
            Err(e) => return Err(e.to_string()),
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

    pub fn send_packet(&mut self, packet: Packet) -> Result<(), String> {
        if let Some(aes) = &self.aes {
            self.socket
                .write_all(
                    &aes.encrypt(
                        packet
                            .try_into(self.compressed)
                            .map_err(|e| e.to_string())?,
                    ),
                )
                .map_err(|e| e.to_string())?;
        } else {
            self.socket
                .write_all(
                    &packet
                        .try_into(self.compressed)
                        .map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?;
        }

        self.socket.flush().map_err(|e| e.to_string())?;

        Ok(())
    }

    fn handshake(&mut self, packet: &Packet) -> Result<(), String> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerHandshakePacketId::Handshake => HandshakePacket::handle(&mut self.state, packet),
        }
    }

    fn status(&mut self, packet: &Packet) -> Result<(), String> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerStatusPacketId::Status => StatusPacket::handle(self),
            ServerStatusPacketId::Ping => PingPacket::handle(self, packet),
        }
    }

    fn login(&mut self, packet: &Packet) -> Result<(), String> {
        match FromPrimitive::from_i32(packet.id).ok_or_else(|| "Unknown packet id")? {
            ServerLoginPacketId::LoginStart => LoginStartPacket::handle(self, packet),
            ServerLoginPacketId::Encryption => EncryptionPacket::handle(self, packet),
            ServerLoginPacketId::LoginPlugin => todo!(),
        }
    }

    pub fn disconnect(&self) -> Result<(), String> {
        let result = self
            .socket
            .shutdown(Shutdown::Both)
            .map_err(|e| e.to_string());

        info!("End tcp client: {}", self.socket_addr);

        result
    }
}

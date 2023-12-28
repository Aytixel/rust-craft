pub mod client;
pub mod server;

pub use client::{ClientConfiguration, ClientHandshake, ClientLogin, ClientPlay, ClientStatus};
pub use server::{ServerConfiguration, ServerLogin, ServerPlay, ServerStatus};

pub enum ServerPacket {
    Status(ServerStatus),
    Login(ServerLogin),
    Configuration(ServerConfiguration),
    Play(ServerPlay),
}

impl TryFrom<ServerPacket> for packet::Packet {
    type Error = packet::Error;

    fn try_from(packet: ServerPacket) -> Result<Self, Self::Error> {
        match packet {
            ServerPacket::Status(packet) => packet.try_into(),
            ServerPacket::Login(packet) => packet.try_into(),
            ServerPacket::Configuration(packet) => packet.try_into(),
            ServerPacket::Play(packet) => packet.try_into(),
        }
    }
}

impl From<ServerStatus> for ServerPacket {
    fn from(packet: ServerStatus) -> Self {
        ServerPacket::Status(packet)
    }
}

impl From<ServerLogin> for ServerPacket {
    fn from(packet: ServerLogin) -> Self {
        ServerPacket::Login(packet)
    }
}

impl From<ServerConfiguration> for ServerPacket {
    fn from(packet: ServerConfiguration) -> Self {
        ServerPacket::Configuration(packet)
    }
}

impl From<ServerPlay> for ServerPacket {
    fn from(packet: ServerPlay) -> Self {
        ServerPacket::Play(packet)
    }
}

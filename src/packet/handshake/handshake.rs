use log::debug;

use crate::client::ClientState;
use crate::data_type::{FromShort, FromString, FromVarInt, Packet};

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    pub next_state: i32,
}

impl HandshakePacket {
    pub fn handle(state: &mut ClientState, packet: &Packet) -> Result<(), String> {
        let handshake_packet = HandshakePacket::try_from(packet.clone())?;

        debug!("{:?}", handshake_packet);

        if handshake_packet.next_state == 1 {
            *state = ClientState::Status;
        } else {
            *state = ClientState::Login;
        }

        Ok(())
    }
}

impl HandshakePacket {
    fn try_from(mut packet: Packet) -> Result<Self, String> {
        Ok(HandshakePacket {
            protocol_version: packet.data.from_varint()?,
            hostname: packet.data.from_packet_string()?,
            port: packet.data.from_short()? as u16,
            next_state: packet.data.from_varint()?,
        })
    }
}

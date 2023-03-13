use log::debug;

use crate::client::ClientState;
use crate::data_type::{FromShort, FromString, FromVarInt, Packet};
use crate::packet::HandshakePacketId;

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    pub next_state: i32,
}

impl HandshakePacket {
    pub fn handle(
        state: &mut ClientState,
        packet: &Packet,
    ) -> Result<Option<Packet>, &'static str> {
        let handshake_packet = HandshakePacket::try_from(packet.clone())?;

        debug!("{:?}", handshake_packet);

        if handshake_packet.next_state == 1 {
            *state = ClientState::Status;
        } else {
            *state = ClientState::Login;
        }

        Ok(None)
    }
}

impl TryFrom<Packet> for HandshakePacket {
    type Error = &'static str;

    fn try_from(mut packet: Packet) -> Result<Self, Self::Error> {
        if packet.id != HandshakePacketId::Handshake as i32 {
            return Err("Wrong packet id");
        }

        Ok(HandshakePacket {
            protocol_version: packet.data.from_varint()?,
            hostname: packet.data.from_packet_string()?,
            port: packet.data.from_short()? as u16,
            next_state: packet.data.from_varint()?,
        })
    }
}

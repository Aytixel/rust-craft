use crate::data_type::{FromString, FromVarInt, Packet};

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    pub next_state: i32,
}

impl TryFrom<Packet> for HandshakePacket {
    type Error = &'static str;

    fn try_from(mut packet: Packet) -> Result<Self, Self::Error> {
        Ok(HandshakePacket {
            protocol_version: packet.data.from_varint()?,
            hostname: packet.data.from_packet_string()?,
            port: u16::from_be_bytes([packet.data.remove(0), packet.data.remove(0)]),
            next_state: packet.data.from_varint()?,
        })
    }
}

use crate::data_type::{Packet, ToString};
use crate::packet::ClientLoginPacketId;

#[derive(Debug)]
pub struct DisconnectPacket;

impl DisconnectPacket {
    pub fn new(reason: String) -> Packet {
        Packet {
            id: ClientLoginPacketId::Disconnect as i32,
            data: reason.to_packet_string(),
        }
    }
}

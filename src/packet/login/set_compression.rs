use crate::client::COMPRESSION_THRESHOLD;
use crate::data_type::{Packet, ToVarInt};
use crate::packet::ClientLoginPacketId;

#[derive(Debug)]
pub struct SetCompressionPacket;

impl SetCompressionPacket {
    pub fn new() -> Packet {
        Packet {
            id: ClientLoginPacketId::SetCompression as i32,
            data: COMPRESSION_THRESHOLD.to_varint(),
        }
    }
}

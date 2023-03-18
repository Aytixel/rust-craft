use uuid::Uuid;

use crate::data_type::{Packet, ToString, ToUuid, ToVarInt};
use crate::packet::ClientLoginPacketId;

#[derive(Debug)]
pub struct LoginSuccessPacket;

impl LoginSuccessPacket {
    pub fn new(uuid: Uuid, username: String) -> Packet {
        let mut data = vec![];

        data.append(&mut uuid.to_uuid());
        data.append(&mut username.to_packet_string());
        data.append(&mut 0.to_varint());

        Packet {
            id: ClientLoginPacketId::LoginSuccess as i32,
            data,
        }
    }
}

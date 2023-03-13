use log::debug;
use serde_json::json;

use crate::data_type::{Packet, ToString};
use crate::packet::StatusPacketId;

#[derive(Debug)]
pub struct StatusPacket;

impl StatusPacket {
    pub fn new(json_response: String) -> Packet {
        Packet {
            id: StatusPacketId::Status as i32,
            data: json_response.to_packet_string(),
        }
    }

    pub fn handle(packet: &Packet) -> Result<Option<Packet>, &'static str> {
        debug!("{:?}", StatusPacket::try_from(packet.clone())?);

        Ok(Some(
            StatusPacket::new(
                json!({
                    "version": {
                        "name": "1.19.3",
                        "protocol": 761
                    },
                    "players": {
                        "max": 0,
                        "online": 0,
                        "sample": []
                    },
                    "description": {
                        "text": "RustCraft"
                    },
                    "favicon": "",
                    "enforcesSecureChat": true
                })
                .to_string(),
            )
            .into(),
        ))
    }
}

impl TryFrom<Packet> for StatusPacket {
    type Error = &'static str;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        if packet.id != StatusPacketId::Status as i32 {
            return Err("Wrong packet id");
        }

        Ok(StatusPacket)
    }
}

use log::debug;
use serde_json::json;

use crate::data_type::{Packet, ToString};
use crate::packet::ClientStatusPacketId;

#[derive(Debug)]
pub struct StatusPacket;

impl StatusPacket {
    pub fn new(json_response: String) -> Packet {
        Packet {
            id: ClientStatusPacketId::Status as i32,
            data: json_response.to_packet_string(),
        }
    }

    pub fn handle() -> Result<Vec<Packet>, &'static str> {
        debug!("{:?}", StatusPacket);

        Ok(vec![StatusPacket::new(
            json!({
                "version": {
                    "name": "1.19.4",
                    "protocol": 762
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
        .into()])
    }
}

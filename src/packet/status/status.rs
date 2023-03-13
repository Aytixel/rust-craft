use log::debug;
use serde_json::json;

use crate::data_type::{Packet, ToString};
use crate::packet::ClientBoundStatusPacketId;

#[derive(Debug)]
pub struct StatusPacket;

impl StatusPacket {
    pub fn new(json_response: String) -> Packet {
        Packet {
            id: ClientBoundStatusPacketId::StatusResponse as i32,
            data: json_response.to_packet_string(),
        }
    }

    pub fn handle() -> Result<Option<Packet>, &'static str> {
        debug!("{:?}", StatusPacket);

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

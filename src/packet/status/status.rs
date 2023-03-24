use log::debug;
use serde_json::json;

use crate::client::Client;
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

    pub fn handle(client: &mut Client) -> Result<(), String> {
        debug!("{:#?}", StatusPacket);

        client
            .send_packet(StatusPacket::new(
                json!({
                    "version": {
                        "name": client.version_info.name,
                        "protocol": client.version_info.protocol
                    },
                    "players": {
                        "max": 0,
                        "online": 0,
                        "sample": []
                    },
                    "description": {
                        "text": "RustCraft"
                    },
                    "favicon": ""
                })
                .to_string(),
            ))
            .map_err(|_| "Error sending StatusPacket")?;

        Ok(())
    }
}

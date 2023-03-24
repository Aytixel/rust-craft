use log::debug;
use uuid::Uuid;

use crate::client::Client;
use crate::data_type::{FromByte, FromString, FromUuid, Packet};
use crate::packet::login::EncryptionPacket;

#[derive(Debug)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}

impl LoginStartPacket {
    pub fn handle(client: &mut Client, packet: &Packet) -> Result<(), String> {
        debug!("{:#?}", LoginStartPacket::try_from(packet.clone())?);

        client
            .send_packet(EncryptionPacket::new(client.encryption_data.clone()))
            .map_err(|_| "Error sending EncryptionPacket")?;

        Ok(())
    }
}

impl LoginStartPacket {
    fn try_from(mut packet: Packet) -> Result<Self, String> {
        let name = packet.data.from_packet_string()?;
        let has_uuid = packet.data.from_byte()? != 0;

        Ok(LoginStartPacket {
            uuid: if has_uuid {
                packet.data.from_uuid()?
            } else {
                Uuid::new_v3(
                    &Uuid::NAMESPACE_URL,
                    format!("OfflinePlayer:{name}").as_bytes(),
                )
            },
            name,
        })
    }
}

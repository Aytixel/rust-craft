use std::rc::Rc;

use log::debug;
use uuid::Uuid;

use crate::{
    data_type::{FromByte, FromString, FromUuid, Packet},
    packet::login::EncryptionPacket,
    server::EncryptionData,
};

#[derive(Debug)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}

impl LoginStartPacket {
    pub fn handle(
        packet: &Packet,
        encryption_data: Rc<EncryptionData>,
    ) -> Result<Vec<Packet>, &'static str> {
        debug!("{:?}", LoginStartPacket::try_from(packet.clone())?);

        Ok(vec![EncryptionPacket::new(encryption_data)])
    }
}

impl TryFrom<Packet> for LoginStartPacket {
    type Error = &'static str;

    fn try_from(mut packet: Packet) -> Result<Self, Self::Error> {
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

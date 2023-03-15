use log::debug;

use crate::data_type::{FromLong, Packet};

#[derive(Debug)]
pub struct PingPacket {
    pub payload: i64,
}

impl PingPacket {
    pub fn handle(packet: &Packet) -> Result<Option<Packet>, &'static str> {
        debug!("{:?}", PingPacket::try_from(packet.clone())?);

        Ok(Some(packet.clone()))
    }
}

impl TryFrom<Packet> for PingPacket {
    type Error = &'static str;

    fn try_from(mut packet: Packet) -> Result<Self, Self::Error> {
        Ok(PingPacket {
            payload: packet.data.from_long()?,
        })
    }
}
use log::debug;

use crate::{
    client::Client,
    data_type::{FromLong, Packet},
};

#[derive(Debug)]
pub struct PingPacket {
    pub payload: i64,
}

impl PingPacket {
    pub fn handle(client: &mut Client, packet: &Packet) -> Result<(), String> {
        debug!("{:#?}", PingPacket::try_from(packet.clone())?);

        client
            .send_packet(packet.clone())
            .map_err(|_| "Error sending PingPacket")?;
        client.disconnect()?;

        Ok(())
    }
}

impl PingPacket {
    fn try_from(mut packet: Packet) -> Result<Self, String> {
        Ok(PingPacket {
            payload: packet.data.from_long()?,
        })
    }
}

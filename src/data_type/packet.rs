use flate2::write::ZlibDecoder;

use super::varint::FromVarInt;

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn try_from(data: &mut Vec<u8>, compressed: bool) -> Result<Self, &'static str> {
        if data.len() == 0 {
            return Err("No data to parse packet");
        }

        let mut data_copy = data.clone();
        let packet_length = data_copy.from_varint()? as usize;

        if data_copy.len() < packet_length {
            return Err("Incomplete packet");
        }

        let mut packet = Packet {
            id: 0,
            data: vec![],
        };

        packet.data = if compressed {
            let data_lenth = data_copy.from_varint()?;

            if data_lenth == 0 {
                data_copy.drain(0..packet_length).collect()
            } else {
                match ZlibDecoder::new(data_copy.drain(0..packet_length).collect()).finish() {
                    Ok(v) => v,
                    Err(_) => return Err("Zlib decoding error"),
                }
            }
        } else {
            data_copy.drain(0..packet_length).collect()
        };

        packet.id = packet.data.from_varint()?;

        if compressed {
            data.from_varint()?;
            data.from_varint()?;
        } else {
            data.from_varint()?;
        }

        data.drain(0..packet_length);

        Ok(packet)
    }
}

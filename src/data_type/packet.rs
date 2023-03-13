use std::io::{self, Write};

use flate2::{write::ZlibDecoder, write::ZlibEncoder, Compression};

use super::{varint::FromVarInt, ToVarInt};

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

    pub fn try_into(mut self, compressed: bool) -> io::Result<Vec<u8>> {
        let mut buffer = vec![];

        buffer.append(&mut self.id.to_varint());
        buffer.append(&mut self.data);

        let mut result_buffer = vec![];

        if compressed {
            let data_length = buffer.len();
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

            encoder.write_all(&buffer)?;
            buffer = encoder.finish()?;

            result_buffer.append(&mut (buffer.len() as i32).to_varint());
            result_buffer.append(&mut (data_length as i32).to_varint());
            result_buffer.append(&mut buffer);
        } else {
            result_buffer.append(&mut (buffer.len() as i32).to_varint());
            result_buffer.append(&mut buffer);
        }

        Ok(result_buffer)
    }
}

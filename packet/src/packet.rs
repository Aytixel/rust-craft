use std::io::Write;

use anyhow::{anyhow, Ok, Result};
use flate2::{write::ZlibDecoder, write::ZlibEncoder, Compression};

use super::{varint::FromVarInt, ToVarInt};

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    pub fn from_bytes(data: &mut Vec<u8>, compressed: bool) -> Result<Self> {
        if data.len() == 0 {
            return Err(anyhow!("No data to parse packet"));
        }

        let mut data_copy = data.clone();
        let mut packet_length = data_copy.from_varint()? as usize;

        if data_copy.len() < packet_length {
            return Err(anyhow!("Incomplete packet"));
        }

        let mut packet = Packet {
            id: 0,
            data: vec![],
        };

        packet.data = if compressed {
            let old_data_length = data_copy.len();
            let data_lenth = data_copy.from_varint()?;

            packet_length -= old_data_length - data_copy.len();

            if data_lenth == 0 {
                data_copy.drain(0..packet_length).collect()
            } else {
                ZlibDecoder::new(data_copy.drain(0..packet_length).collect()).finish()?
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

    pub fn into_bytes(self, compressed: bool, compression_threshold: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![];

        buffer.extend(self.id.to_varint());
        buffer.extend(self.data);

        let mut result_buffer = vec![];
        let mut data_length = (buffer.len() as i32).to_varint();

        if compressed {
            if buffer.len() >= compression_threshold {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

                encoder.write_all(&buffer)?;
                buffer = encoder.finish()?;
            } else {
                data_length = vec![0];
            }

            result_buffer.extend(((data_length.len() + buffer.len()) as i32).to_varint());
        }

        result_buffer.extend(data_length);
        result_buffer.extend(buffer);

        Ok(result_buffer)
    }
}

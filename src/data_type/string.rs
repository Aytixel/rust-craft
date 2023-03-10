use super::varint::{FromVarInt, ToVarInt};

pub trait FromString {
    fn from_packet_string(&mut self) -> Result<String, &'static str>;
}

impl FromString for Vec<u8> {
    fn from_packet_string(&mut self) -> Result<String, &'static str> {
        let length = self.from_varint()?;

        Ok(String::from_utf8_lossy(self.drain(0..length as usize).as_slice()).to_string())
    }
}

pub trait ToString {
    fn to_packet_string(self) -> Vec<u8>;
}

impl ToString for String {
    fn to_packet_string(self) -> Vec<u8> {
        let mut data = (self.len() as i32).to_varint();

        data.append(&mut self.as_bytes().to_vec());
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_packet_string() {
        assert_eq!(
            vec![9, 0, 0, 0, 0, 0, 1, 1, 2, 10]
                .from_packet_string()
                .unwrap()
                .as_bytes(),
            vec![0, 0, 0, 0, 0, 1, 1, 2, 10]
        );
    }

    #[test]
    fn to_packet_string() {
        assert_eq!(
            String::from_utf8_lossy(&vec![0, 0, 0, 0, 0, 1, 1, 2, 10])
                .to_string()
                .to_packet_string(),
            vec![9, 0, 0, 0, 0, 0, 1, 1, 2, 10]
        );
    }
}

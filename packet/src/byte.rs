use anyhow::{anyhow, Result};

pub trait FromByte {
    fn from_byte(&mut self) -> Result<i8>;
}

impl FromByte for Vec<u8> {
    fn from_byte(&mut self) -> Result<i8> {
        if self.len() < 1 {
            return Err(anyhow!("Not enough data to parse Byte"));
        }

        Ok(self.remove(0) as i8)
    }
}

pub trait ToByte {
    fn to_byte(self) -> Vec<u8>;
}

impl ToByte for i8 {
    fn to_byte(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ToByte for u8 {
    fn to_byte(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_byte() {
        assert_eq!(
            vec![].from_byte().unwrap_err().to_string(),
            anyhow!("Not enough data to parse Byte").to_string()
        );
        assert_eq!(vec![1].from_byte().unwrap(), 1);
        assert_eq!(vec![89].from_byte().unwrap(), 89);
    }

    #[test]
    fn to_byte() {
        assert_eq!(80i8.to_byte(), vec![80]);
    }
}

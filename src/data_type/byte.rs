pub trait FromByte {
    fn from_byte(&mut self) -> Result<i8, &'static str>;
}

impl FromByte for Vec<u8> {
    fn from_byte(&mut self) -> Result<i8, &'static str> {
        if self.len() < 1 {
            return Err("Not enough data to parse Byte");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_byte() {
        assert_eq!(vec![].from_byte(), Err("Not enough data to parse Byte"));
        assert_eq!(vec![1].from_byte().unwrap(), 1);
        assert_eq!(vec![89].from_byte().unwrap(), 89);
    }

    #[test]
    fn to_byte() {
        assert_eq!(80.to_byte(), vec![80]);
    }
}

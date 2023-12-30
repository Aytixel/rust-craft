use anyhow::{anyhow, Result};

pub trait FromShort {
    fn from_short(&mut self) -> Result<i16>;
}

impl FromShort for Vec<u8> {
    fn from_short(&mut self) -> Result<i16> {
        if self.len() < 2 {
            return Err(anyhow!("Not enough data to parse Short"));
        }

        Ok(i16::from_be_bytes([self.remove(0), self.remove(0)]))
    }
}

pub trait ToShort {
    fn to_short(self) -> Vec<u8>;
}

impl ToShort for i16 {
    fn to_short(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl ToShort for u16 {
    fn to_short(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_short() {
        assert_eq!(
            vec![1].from_short().unwrap_err().to_string(),
            anyhow!("Not enough data to parse Short").to_string()
        );
        assert_eq!(vec![1, 1].from_short().unwrap(), 257);
    }

    #[test]
    fn to_short() {
        assert_eq!(18080i16.to_short(), vec![70, 160]);
    }
}

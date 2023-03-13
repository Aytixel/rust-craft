pub trait FromLong {
    fn from_long(&mut self) -> Result<i64, &'static str>;
}

impl FromLong for Vec<u8> {
    fn from_long(&mut self) -> Result<i64, &'static str> {
        if self.len() < 2 {
            return Err("Not enough data to parse Long");
        }

        Ok(i64::from_be_bytes([
            self.remove(0),
            self.remove(0),
            self.remove(0),
            self.remove(0),
            self.remove(0),
            self.remove(0),
            self.remove(0),
            self.remove(0),
        ]))
    }
}

pub trait ToLong {
    fn to_long(self) -> Vec<u8>;
}

impl ToLong for i64 {
    fn to_long(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_long() {
        assert_eq!(vec![1].from_long(), Err("Not enough data to parse Long"));
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1].from_long().unwrap(),
            72340172838076673
        );
    }

    #[test]
    fn to_long() {
        assert_eq!(18080.to_long(), vec![0, 0, 0, 0, 0, 0, 70, 160]);
    }
}

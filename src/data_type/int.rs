pub trait FromInt {
    fn from_int(&mut self) -> Result<i32, &'static str>;
}

impl FromInt for Vec<u8> {
    fn from_int(&mut self) -> Result<i32, &'static str> {
        if self.len() < 4 {
            return Err("Not enough data to parse Int");
        }

        Ok(i32::from_be_bytes([
            self.remove(0),
            self.remove(0),
            self.remove(0),
            self.remove(0),
        ]))
    }
}

pub trait ToInt {
    fn to_int(self) -> Vec<u8>;
}

impl ToInt for i32 {
    fn to_int(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_int() {
        assert_eq!(vec![1].from_int(), Err("Not enough data to parse Int"));
        assert_eq!(vec![1, 1, 1, 1].from_int().unwrap(), 16843009);
    }

    #[test]
    fn to_int() {
        assert_eq!(18080.to_int(), vec![0, 0, 70, 160]);
    }
}

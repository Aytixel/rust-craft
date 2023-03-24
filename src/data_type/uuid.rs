use uuid::Uuid;

pub trait FromUuid {
    fn from_uuid(&mut self) -> Result<Uuid, String>;
}

impl FromUuid for Vec<u8> {
    fn from_uuid(&mut self) -> Result<Uuid, String> {
        if self.len() < 16 {
            return Err("Not enough data to parse Uuid".to_string());
        }

        let uuid = Uuid::from_slice(&self[..16]).map_err(|e| e.to_string())?;

        self.drain(..16);

        Ok(uuid)
    }
}

pub trait ToUuid {
    fn to_uuid(self) -> Vec<u8>;
}

impl ToUuid for Uuid {
    fn to_uuid(self) -> Vec<u8> {
        self.into_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn from_uuid() {
        assert_eq!(
            vec![1].from_uuid(),
            Err("Not enough data to parse Uuid".to_string())
        );
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
                .from_uuid()
                .unwrap(),
            uuid!("01010101-0101-0101-0101-010101010101")
        );
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 89]
                .from_uuid()
                .unwrap(),
            uuid!("01010101-0101-0101-0101-010101010159")
        );
    }

    #[test]
    fn to_uuid() {
        assert_eq!(
            uuid!("00000000-0000-0000-0000-ffff00000000").to_uuid(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0]
        );
    }
}

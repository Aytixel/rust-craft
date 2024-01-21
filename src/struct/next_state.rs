use anyhow::anyhow;
use packet::FromVarInt;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

impl TryFrom<&mut Vec<u8>> for NextState {
    type Error = anyhow::Error;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        match value.from_varint()? {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            _ => Err(anyhow!("Can't deserialize next state")),
        }
    }
}

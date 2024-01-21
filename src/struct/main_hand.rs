use anyhow::anyhow;
use packet::FromVarInt;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

impl TryFrom<&mut Vec<u8>> for MainHand {
    type Error = anyhow::Error;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        match value.from_varint()? {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err(anyhow!("Can't deserialize Main Hand")),
        }
    }
}

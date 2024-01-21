use anyhow::anyhow;
use packet::FromVarInt;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

impl TryFrom<&mut Vec<u8>> for ChatMode {
    type Error = anyhow::Error;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        match value.from_varint()? {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::CommandsOnly),
            2 => Ok(Self::Hidden),
            _ => Err(anyhow!("Can't deserialize Chat Mode")),
        }
    }
}

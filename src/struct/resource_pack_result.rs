use anyhow::anyhow;
use packet::FromVarInt;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum ResourcePackResult {
    SuccessfullyDownloaded = 0,
    Declined = 1,
    FailedToDownload = 2,
    Accepted = 3,
    Downloaded = 4,
    InvalidUrl = 5,
    FailedToReload = 6,
    Discarded = 7,
}

impl TryFrom<&mut Vec<u8>> for ResourcePackResult {
    type Error = anyhow::Error;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        match value.from_varint()? {
            0 => Ok(Self::SuccessfullyDownloaded),
            1 => Ok(Self::Declined),
            2 => Ok(Self::FailedToDownload),
            3 => Ok(Self::Accepted),
            4 => Ok(Self::Downloaded),
            5 => Ok(Self::InvalidUrl),
            6 => Ok(Self::FailedToReload),
            7 => Ok(Self::Discarded),
            _ => Err(anyhow!("Can't deserialize Resource Pack Result")),
        }
    }
}

use anyhow::anyhow;
use bitflags::bitflags;
use packet::FromByte;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DiplayedSkinParts: u8 {
        const Cape = 0x01;
        const Jacket = 0x02;
        const LeftSleeve = 0x04;
        const RightSleeve = 0x08;
        const LeftPantsLeg = 0x10;
        const RightPantsLeg = 0x20;
        const Hat = 0x40;

        const All = Self::Cape.bits()
            | Self::Jacket.bits()
            | Self::LeftSleeve.bits()
            | Self::RightSleeve.bits()
            | Self::LeftPantsLeg.bits()
            | Self::RightPantsLeg.bits()
            | Self::Hat.bits();
    }
}

impl TryFrom<&mut Vec<u8>> for DiplayedSkinParts {
    type Error = anyhow::Error;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bits(value.from_byte()? as u8)
            .ok_or(anyhow!("Can't deserialize displayed skin parts flags"))
    }
}

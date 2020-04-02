use crate::le::advertisement::{
    AdStructureType, AdType, ConstAdStructType, UnpackableAdStructType,
};
use crate::PackError;
use std::fmt::{Error, Formatter};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug, Hash)]
pub struct TxPowerLevel {
    pub dbm: i8,
}
impl core::fmt::Display for TxPowerLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}dbm", self.dbm)
    }
}
impl TxPowerLevel {
    pub const AD_TYPE: AdType = AdType::TxPowerLevel;

    pub const BYTE_LEN: usize = 1;
    pub fn new(dbm: i8) -> TxPowerLevel {
        TxPowerLevel { dbm }
    }
}

impl AdStructureType for TxPowerLevel {
    fn ad_type(&self) -> AdType {
        Self::AD_TYPE
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.dbm as u8;
        Ok(())
    }
}
impl UnpackableAdStructType for TxPowerLevel {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if ad_type != Self::AD_TYPE {
            Err(PackError::InvalidFields)
        } else {
            PackError::expect_length(Self::BYTE_LEN, buf)?;
            Ok(Self::new(buf[0] as i8))
        }
    }
}
impl ConstAdStructType for TxPowerLevel {
    const AD_TYPE: AdType = AdType::TxPowerLevel;
}

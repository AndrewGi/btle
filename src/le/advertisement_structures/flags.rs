use crate::le::advertisement::{AdStructureType, AdType, UnpackableAdStructType};
use crate::PackError;
use core::convert::TryFrom;
use std::convert::TryInto;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(u8)]
pub enum BitFlags {
    LELimitedDiscoverableMode = 0,
    LEGeneralDiscoverableMode = 1,
    BrEdrNotSupported = 2,
    SimultaneousLEAndBrEdrController = 3,
    SimultaneousLEAndBrEdrHost = 4,
}
pub struct Flags(u8);
impl Flags {
    pub const FLAGS_MAX: u8 = (1 << 4_u8) - 1;
    pub const AD_TYPE: AdType = AdType::Flags;
    pub const BYTE_LEN: usize = 1;
}
impl From<Flags> for u8 {
    fn from(f: Flags) -> Self {
        f.0
    }
}
impl TryFrom<u8> for Flags {
    type Error = crate::ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Flags::FLAGS_MAX {
            Err(crate::ConversionError(()))
        } else {
            Ok(Flags(value))
        }
    }
}
impl AdStructureType for Flags {
    fn ad_type(&self) -> AdType {
        Self::AD_TYPE
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.0;
        Ok(())
    }
}
impl UnpackableAdStructType for Flags {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if ad_type != Self::AD_TYPE {
            return Err(PackError::BadOpcode);
        }
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0].try_into().map_err(|_| PackError::bad_index(0))
    }
}

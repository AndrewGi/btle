use crate::bytes::Storage;
use crate::le::advertisement::{AdStructureType, AdType, UnpackableAdStructType};
use crate::PackError;

pub mod flags;
pub mod local_name;
pub mod manufacturer_data;
pub mod tx_power_level;

pub enum Structs<Buf> {
    Flags(flags::Flags),
    LocalName(local_name::LocalName<Buf>),
    ManufacturerData(manufacturer_data::ManufacturerSpecificData<Buf>),
    TxPowerLevel(tx_power_level::TxPowerLevel),
}
impl<Buf: AsRef<[u8]>> AdStructureType for Structs<Buf> {
    fn ad_type(&self) -> AdType {
        match self {
            Structs::Flags(_) => flags::Flags::AD_TYPE,
            Structs::LocalName(l) => l.ad_type(),
            Structs::ManufacturerData(_) => {
                manufacturer_data::ManufacturerSpecificData::<Buf>::AD_TYPE
            }
            Structs::TxPowerLevel(_) => tx_power_level::TxPowerLevel::AD_TYPE,
        }
    }

    fn byte_len(&self) -> usize {
        match self {
            Structs::Flags(f) => f.byte_len(),
            Structs::LocalName(l) => l.byte_len(),
            Structs::ManufacturerData(d) => d.byte_len(),
            Structs::TxPowerLevel(t) => t.byte_len(),
        }
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        match self {
            Structs::Flags(f) => f.pack_into(buf),
            Structs::LocalName(l) => l.pack_into(buf),
            Structs::ManufacturerData(d) => d.pack_into(buf),
            Structs::TxPowerLevel(t) => t.pack_into(buf),
        }
    }
}
impl<Buf: Storage<u8>> UnpackableAdStructType for Structs<Buf> {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        match ad_type {
            AdType::CompleteLocalName | AdType::ShortenLocalName => Ok(Structs::LocalName(
                local_name::LocalName::unpack_from(ad_type, buf)?,
            )),
            AdType::Flags => Ok(Structs::Flags(flags::Flags::unpack_from(ad_type, buf)?)),
            AdType::ManufacturerData => Ok(Structs::ManufacturerData(
                manufacturer_data::ManufacturerSpecificData::unpack_from(ad_type, buf)?,
            )),
            AdType::TxPowerLevel => Ok(Structs::TxPowerLevel(
                tx_power_level::TxPowerLevel::unpack_from(ad_type, buf)?,
            )),
            _ => Err(PackError::BadOpcode),
        }
    }
}

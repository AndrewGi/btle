//! BLE Advertisements. Provides processing of Advertisement Structs.

use crate::bytes::{StaticBuf, Storage};
use crate::PackError;
use core::convert::TryFrom;
use core::mem;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct AdStructureError(());

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
#[repr(u8)]
pub enum AdType {
    Flags = 0x01,
    IncompleteList16bitUUID = 0x02,
    CompleteList16bitUUID = 0x03,
    IncompleteList32bitUUID = 0x04,
    CompleteList32bitUUID = 0x05,
    IncompleteList128bitUUID = 0x06,
    CompleteList128bitUUID = 0x07,
    ShortenLocalName = 0x08,
    CompleteLocalName = 0x09,
    TxPowerLevel = 0x0A,
    ClassOfDevice = 0x0D,
    SimplePairingHashC = 0x0E,
    SimplePairingRandomizerR = 0x0F,
    SecurityManagerTKValue = 0x10,
    SecurityManagerOOBFlags = 0x11,
    SlaveConnectionIntervalRange = 0x12,
    List16bitSolicitationUUID = 0x14,
    List128bitSolicitationUUID = 0x15,
    ServiceData = 0x16,
    PublicTargetAddress = 0x17,
    RandomTargetAddress = 0x18,
    Appearance = 0x19,
    AdvertisingInterval = 0x1A,
    LEDeviceAddress = 0x1B,
    LERole = 0x1C,
    SimplePairingHashC256 = 0x1D,
    SimplePairingHashRandomizerR256 = 0x1E,
    List32bitSolicitationUUID = 0x1F,
    ServiceData32bitUUID = 0x20,
    ServiceData128bitUUID = 0x21,
    LESecureConfirmValue = 0x22,
    LEConfirmRandomValue = 0x23,
    URI = 0x24,
    IndoorPositioning = 0x25,
    TransportDiscoveryData = 0x26,
    LESupportedFeatures = 0x27,
    ChannelMapUpdateIndication = 0x28,
    PbAdv = 0x29,
    MeshPDU = 0x2A,
    MeshBeacon = 0x2B,
    BIGInfo = 0x2C,
    BroadcastCode = 0x2D,
    Information3DData = 0x3D,
    ManufacturerData = 0xFF,
}
impl From<AdType> for u8 {
    fn from(a: AdType) -> Self {
        a as u8
    }
}
impl TryFrom<u8> for AdType {
    type Error = AdStructureError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(AdType::Flags),
            0x02 => Ok(AdType::IncompleteList16bitUUID),
            0x03 => Ok(AdType::CompleteList16bitUUID),
            0x04 => Ok(AdType::IncompleteList32bitUUID),
            0x05 => Ok(AdType::CompleteList32bitUUID),
            0x06 => Ok(AdType::IncompleteList128bitUUID),
            0x07 => Ok(AdType::CompleteList128bitUUID),
            0x08 => Ok(AdType::ShortenLocalName),
            0x09 => Ok(AdType::CompleteLocalName),
            0x0A => Ok(AdType::TxPowerLevel),
            0x0D => Ok(AdType::ClassOfDevice),
            0x0E => Ok(AdType::SimplePairingHashC),
            0x0F => Ok(AdType::SimplePairingRandomizerR),
            0x10 => Ok(AdType::SecurityManagerTKValue),
            0x11 => Ok(AdType::SecurityManagerOOBFlags),
            0x12 => Ok(AdType::SlaveConnectionIntervalRange),
            0x14 => Ok(AdType::List16bitSolicitationUUID),
            0x15 => Ok(AdType::List128bitSolicitationUUID),
            0x16 => Ok(AdType::ServiceData),
            0x17 => Ok(AdType::PublicTargetAddress),
            0x18 => Ok(AdType::RandomTargetAddress),
            0x19 => Ok(AdType::Appearance),
            0x1A => Ok(AdType::AdvertisingInterval),
            0x1B => Ok(AdType::LEDeviceAddress),
            0x1C => Ok(AdType::LERole),
            0x1D => Ok(AdType::SimplePairingHashC256),
            0x1E => Ok(AdType::SimplePairingHashRandomizerR256),
            0x1F => Ok(AdType::List32bitSolicitationUUID),
            0x20 => Ok(AdType::ServiceData32bitUUID),
            0x21 => Ok(AdType::ServiceData128bitUUID),
            0x22 => Ok(AdType::LESecureConfirmValue),
            0x23 => Ok(AdType::LEConfirmRandomValue),
            0x24 => Ok(AdType::URI),
            0x25 => Ok(AdType::IndoorPositioning),
            0x26 => Ok(AdType::TransportDiscoveryData),
            0x27 => Ok(AdType::LESupportedFeatures),
            0x28 => Ok(AdType::ChannelMapUpdateIndication),
            0x29 => Ok(AdType::PbAdv),
            0x2A => Ok(AdType::MeshPDU),
            0x2B => Ok(AdType::MeshBeacon),
            0x2C => Ok(AdType::BIGInfo),
            0x2D => Ok(AdType::BroadcastCode),
            0x3D => Ok(AdType::Information3DData),
            0xFF => Ok(AdType::ManufacturerData),
            _ => Err(AdStructureError(())),
        }
    }
}
pub trait AdStructureType {
    fn ad_type(&self) -> AdType;
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
}
pub trait UnpackableAdStructType: AdStructureType {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
}
pub const MAX_AD_LEN: usize = 30;
pub type StaticAdvBuffer = StaticBuf<u8, [u8; MAX_ADV_LEN]>;
pub type StaticAdvStructBuf = StaticBuf<u8, [u8; MAX_AD_LEN]>;
pub struct RawAdStructureBuffer<StructBuf = StaticAdvStructBuf> {
    pub ad_type: AdType,
    pub buf: StructBuf,
}
impl<StructBuf> RawAdStructureBuffer<StructBuf> {
    pub fn new(ad_type: AdType, buf: StructBuf) -> Self {
        Self { ad_type, buf }
    }
}
impl<StructBuf: AsRef<[u8]>> AdStructureType for RawAdStructureBuffer<StructBuf> {
    fn ad_type(&self) -> AdType {
        self.ad_type
    }

    fn byte_len(&self) -> usize {
        self.buf.as_ref().len()
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.buf.as_ref().len(), buf)?;
        buf.copy_from_slice(self.buf.as_ref());
        Ok(())
    }
}
impl<StructBuf: Storage<u8>> UnpackableAdStructType for RawAdStructureBuffer<StructBuf> {
    fn unpack_from(ad_type: AdType, buf: &[u8]) -> Result<Self, PackError> {
        if buf.len() > MAX_AD_LEN {
            PackError::expect_length(MAX_AD_LEN, buf)?;
        }
        Ok(Self::new(ad_type, StructBuf::from_slice(buf)))
    }
}
pub const MAX_ADV_LEN: usize = 31;
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Default, Hash, Debug)]
pub struct RawAdvertisement<Buf = StaticAdvBuffer>(pub Buf);
impl RawAdvertisement<StaticAdvBuffer> {
    /// Inserts a `AdStructure` into a `RawAdvertisement`
    pub fn insert<AdStruct: AdStructureType>(
        &mut self,
        ad_struct: &AdStruct,
    ) -> Result<(), PackError> {
        let current_len = self.0.len();
        let ad_struct_len = ad_struct.byte_len();
        // ad_struct_len + 1 (for AdType) + 1 (for byte len as u8)
        let total_struct_len = ad_struct_len + 1 + 1;
        if self.0.space_left() < total_struct_len {
            return Err(PackError::BadLength {
                expected: total_struct_len + current_len,
                got: StaticAdvBuffer::max_size(),
            });
        }
        self.0.resize(current_len + total_struct_len);
        let len = ad_struct.byte_len();
        // The AdStruct byte len should always be less than MAX_AD_LEN (30) and so it should always
        // be able to fit in a u8. If the usize -> u8 conversion fails, then theres something really
        // wrong with the ad structure.
        let len_u8 = u8::try_from(len).map_err(|_| PackError::InvalidFields)?;
        ad_struct.pack_into(&mut self.0.as_mut()[current_len + 2..])?;
        self.0.as_mut()[current_len] = ad_struct.ad_type().into();
        self.0.as_mut()[current_len + 1] = len_u8;
        Ok(())
    }
}
impl<Buf: AsRef<[u8]>> RawAdvertisement<Buf> {
    pub fn iter(&self) -> AdStructureIterator<'_> {
        AdStructureIterator {
            data: self.as_ref(),
        }
    }
}
impl<Buf: AsRef<[u8]>> AsRef<[u8]> for RawAdvertisement<Buf> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub struct OutgoingAdvertisement {
    adv: RawAdvertisement,
}
pub struct AdStructureIterator<'a> {
    data: &'a [u8],
}

impl<'a> Iterator for AdStructureIterator<'a> {
    type Item = RawAdStructureBuffer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < 2 {
            return None;
        }
        let d = mem::replace(&mut self.data, &[]);
        let len = usize::from(d[0]);
        let (data, rest) = d.split_at(len + 1);
        self.data = rest;
        let ad_type = AdType::try_from(data[1]).ok()?;
        // Drop the len and ad_type from the front of the ad structure.
        let data = &data[2..];
        Some(RawAdStructureBuffer::new(
            ad_type,
            StaticAdvStructBuf::from_slice(data),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::AdType;
    use core::convert::TryFrom;
    #[test]
    fn test_ad_type_try_into() {
        for i in 0u8..=255u8 {
            match AdType::try_from(i) {
                Ok(t) => assert_eq!(u8::from(t), i),
                Err(_) => (),
            }
        }
    }
}

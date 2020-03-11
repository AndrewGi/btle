use crate::bytes::Storage;
use crate::{ConversionError, PackError};
use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
#[repr(u8)]
pub enum PacketType {
    Command = 0x01,
    ACLData = 0x02,
    SCOData = 0x03,
    Event = 0x04,
    Vendor = 0xFF,
}
impl From<PacketType> for u8 {
    fn from(packet_type: PacketType) -> Self {
        packet_type as u8
    }
}
impl From<PacketType> for u32 {
    fn from(packet_type: PacketType) -> Self {
        packet_type as u32
    }
}
impl TryFrom<u8> for PacketType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(PacketType::Command),
            0x02 => Ok(PacketType::ACLData),
            0x03 => Ok(PacketType::SCOData),
            0x04 => Ok(PacketType::Event),
            0xFF => Ok(PacketType::Vendor),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct RawPacket<Buf: AsRef<[u8]>> {
    pub packet_type: PacketType,
    pub buf: Buf,
}
impl<Buf: AsRef<[u8]>> RawPacket<Buf> {
    pub fn as_ref(&self) -> RawPacket<&[u8]> {
        RawPacket {
            packet_type: self.packet_type,
            buf: self.buf.as_ref(),
        }
    }
}
impl<Buf: AsRef<[u8]>> RawPacket<Buf> {
    pub fn clone_buf<S: Storage<u8>>(&self) -> RawPacket<S> {
        RawPacket {
            packet_type: self.packet_type,
            buf: S::from_slice(self.buf.as_ref()),
        }
    }
}
impl<'a> TryFrom<&'a [u8]> for RawPacket<&'a [u8]> {
    type Error = ConversionError;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(RawPacket {
            packet_type: (*buf.get(0).ok_or(ConversionError(()))?).try_into()?,
            buf: &buf[1..],
        })
    }
}
pub trait Packet {
    const PACKET_TYPE: PacketType;
    fn packet_byte_len(&self) -> usize;
    fn packet_pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
    fn packet_unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
    /// Automatically trims `buf` to the correct len.
    fn packet_pack_full(&self, buf: &mut [u8]) -> Result<usize, PackError> {
        let full = self.packet_byte_len() + 1;
        let buf = &mut buf[..full];
        PackError::expect_length(full, buf)?;
        self.packet_pack_into(&mut buf[1..])?;
        buf[0] = Self::PACKET_TYPE.into();
        Ok(full)
    }
    fn try_from<S: AsRef<[u8]>>(value: &RawPacket<S>) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if Self::PACKET_TYPE != value.packet_type {
            Err(PackError::BadOpcode)
        } else {
            Self::packet_unpack_from(value.buf.as_ref())
        }
    }
}

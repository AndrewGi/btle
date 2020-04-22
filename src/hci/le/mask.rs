//! LE [`SetEventMask`] and [`EventMask`] for dealing with LE event masks.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, StatusReturn};
use crate::hci::le::{LEControllerOpcode, MetaEventCode};
use crate::hci::Opcode;
use crate::ConversionError;
use crate::PackError;
use core::convert::TryFrom;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MetaEventMask(u64);
impl MetaEventMask {
    const DEFAULT: MetaEventMask = MetaEventMask(0x1F);
    const ZEROED: MetaEventMask = MetaEventMask(0);
    const BYTE_LEN: usize = 8;
    pub const fn zeroed() -> MetaEventMask {
        Self::ZEROED
    }
    pub fn new(mask: u64) -> MetaEventMask {
        MetaEventMask(mask)
    }
    pub fn enable_event(&mut self, event: MetaEventCode) {
        self.0 |= 1u64 << Self::event_pos(event)
    }
    pub fn disable_event(&mut self, event: MetaEventCode) {
        self.0 &= !(1u64 << Self::event_pos(event))
    }
    fn event_pos(event: MetaEventCode) -> u64 {
        u64::from(u8::from(event) - 1)
    }
    pub fn get_event(&self, event: MetaEventCode) -> bool {
        self.0 & (1u64 << Self::event_pos(event)) != 0
    }
}
impl From<MetaEventMask> for u64 {
    fn from(m: MetaEventMask) -> Self {
        m.0
    }
}
impl TryFrom<u64> for MetaEventMask {
    type Error = ConversionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // Might do validity checking of the bits.
        Ok(MetaEventMask(value))
    }
}
impl Default for MetaEventMask {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct SetMetaEventMask(pub MetaEventMask);

impl Command for SetMetaEventMask {
    type Return = CommandComplete<StatusReturn>;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetEventMask.into()
    }

    fn byte_len(&self) -> usize {
        MetaEventMask::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(MetaEventMask::BYTE_LEN, buf)?;
        buf.copy_from_slice(&(self.0).0.to_bytes_le());
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(MetaEventMask::BYTE_LEN, buf)?;
        Ok(SetMetaEventMask(
            MetaEventMask::try_from(u64::from_bytes_le(buf).expect("length checked above"))
                .map_err(|_| PackError::bad_index(0))?,
        ))
    }
}

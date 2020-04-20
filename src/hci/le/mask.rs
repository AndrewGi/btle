//! LE [`SetEventMask`] and [`EventMask`] for dealing with LE event masks.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::StatusReturn;
use crate::hci::le::{LEControllerOpcode, MetaEventCode};
use crate::hci::Opcode;
use crate::ConversionError;
use crate::PackError;
use core::convert::TryFrom;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EventMask(u64);
impl EventMask {
    const DEFAULT: EventMask = EventMask(0x1F);
    const ZEROED: EventMask = EventMask(0);
    const BYTE_LEN: usize = 8;
    pub const fn zeroed() -> EventMask {
        Self::ZEROED
    }
    pub fn new(mask: u64) -> EventMask {
        EventMask(mask)
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
impl From<EventMask> for u64 {
    fn from(m: EventMask) -> Self {
        m.0
    }
}
impl TryFrom<u64> for EventMask {
    type Error = ConversionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // Might do validity checking of the bits.
        Ok(EventMask(value))
    }
}
impl Default for EventMask {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct SetEventMask {
    pub mask: EventMask,
}
impl Command for SetEventMask {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetEventMask.into()
    }

    fn byte_len(&self) -> usize {
        EventMask::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(EventMask::BYTE_LEN, buf)?;
        buf.copy_from_slice(&self.mask.0.to_bytes_le());
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(EventMask::BYTE_LEN, buf)?;
        Ok(SetEventMask {
            mask: EventMask::try_from(u64::from_bytes_le(buf).expect("length checked above"))
                .map_err(|_| PackError::bad_index(0))?,
        })
    }
}

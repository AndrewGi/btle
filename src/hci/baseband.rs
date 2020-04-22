use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, StatusReturn};
use crate::hci::{Opcode, OCF, OGF};
use crate::PackError;
use std::convert::TryInto;

pub enum ControllerBasebandOpcode {
    SetEventMask = 0x0001,
    Reset = 0x0003,
    SetEventFilter = 0x0005,
    Flush = 0x0008,
    ReadPIN = 0x0009,
    WritePIN = 0x000A,
    ReadStoredLinkKey = 0x000D,
}
impl From<ControllerBasebandOpcode> for u16 {
    fn from(opcode: ControllerBasebandOpcode) -> Self {
        opcode as u16
    }
}
impl From<ControllerBasebandOpcode> for OCF {
    fn from(opcode: ControllerBasebandOpcode) -> Self {
        OCF::new(opcode.into())
    }
}
impl From<ControllerBasebandOpcode> for Opcode {
    fn from(ocf: ControllerBasebandOpcode) -> Self {
        Opcode(OGF::HCIControlBaseband, ocf.into())
    }
}
pub struct Reset;
impl Reset {
    pub const OPCODE: ControllerBasebandOpcode = ControllerBasebandOpcode::Reset;
}
impl Command for Reset {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        Self::OPCODE.into()
    }

    fn byte_len(&self) -> usize {
        0
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(0, buf)
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(0, buf)?;
        Ok(Reset)
    }
}
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq, Default)]
pub struct EventMask(pub u64);

#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq, Default)]
pub struct SetEventMask(pub EventMask);

impl SetEventMask {
    pub const BYTE_LEN: usize = 8;
    pub const OPCODE: ControllerBasebandOpcode = ControllerBasebandOpcode::SetEventMask;
}
impl Command for SetEventMask {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        Self::OPCODE.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf.copy_from_slice((self.0).0.to_le_bytes().as_ref());
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        Ok(SetEventMask(EventMask(u64::from_le_bytes(
            buf.try_into().map_err(|_| PackError::BadLength {
                expected: Self::BYTE_LEN,
                got: buf.len(),
            })?,
        ))))
    }
}

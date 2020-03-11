use crate::hci::command::Command;
use crate::hci::event::StatusReturn;
use crate::hci::le::LEControllerOpcode;
use crate::hci::Opcode;
use crate::PackError;
use core::convert::TryInto;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetAdvertisingEnable {
    pub is_enabled: bool,
}
const SET_ADVERTISING_ENABLE_LEN: usize = 1;
impl Command for SetAdvertisingEnable {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetAdvertisingEnable.into()
    }

    fn byte_len(&self) -> usize {
        SET_ADVERTISING_ENABLE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(SET_ADVERTISING_ENABLE_LEN, buf)?;
        buf[0] = self.is_enabled.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(SET_ADVERTISING_ENABLE_LEN, buf)?;
        match buf[0] {
            0 => Ok(Self { is_enabled: false }),
            1 => Ok(Self { is_enabled: true }),
            _ => Err(PackError::bad_index(0)),
        }
    }
}
const ADVERTISING_DATA_MAX_LEN: usize = 0x1F;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetAdvertisingData {
    data: [u8; ADVERTISING_DATA_MAX_LEN],
    len: u8,
}

impl Command for SetAdvertisingData {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetAdvertisingData.into()
    }

    fn byte_len(&self) -> usize {
        usize::from(self.len) + 1
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.byte_len(), buf)?;
        buf[0] = self.len;
        let l = usize::from(self.len);
        buf[1..][..l].copy_from_slice(&self.data[..l]);
        Ok(())
    }

    fn unpack_from(_buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
impl SetAdvertisingData {
    pub fn new(data: &[u8]) -> SetAdvertisingData {
        assert!(data.len() <= ADVERTISING_DATA_MAX_LEN);
        let mut buf = [0_u8; ADVERTISING_DATA_MAX_LEN];
        buf[..data.len()].copy_from_slice(data);
        SetAdvertisingData {
            data: buf,
            len: data.len().try_into().expect("data max len 0x1F"),
        }
    }
}

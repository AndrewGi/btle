//! LE [`SetScanEnable`], [`SetScanParameters`], and other primitive scan types.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, StatusReturn};
use crate::hci::le::LEControllerOpcode;
use crate::hci::Opcode;
use crate::le::scan::ScanParameters;
use crate::PackError;
use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetScanEnable {
    pub is_enabled: bool,
    pub filter_duplicates: bool,
}
impl SetScanEnable {
    pub const BYTE_LEN: usize = 2;
}
impl Command for SetScanEnable {
    type Return = CommandComplete<StatusReturn>;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetScanEnable.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.is_enabled.into();
        buf[1] = self.filter_duplicates.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        let is_enabled = match buf[0] {
            0 => false,
            1 => true,
            _ => return Err(PackError::bad_index(0)),
        };
        let filter_duplicates = match buf[1] {
            0 => false,
            1 => true,
            _ => return Err(PackError::bad_index(1)),
        };
        Ok(Self {
            is_enabled,
            filter_duplicates,
        })
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct SetScanParameters(pub ScanParameters);
impl SetScanParameters {
    pub const DEFAULT: SetScanParameters = SetScanParameters(ScanParameters::DEFAULT);
    pub fn new(parameters: ScanParameters) -> Self {
        Self(parameters)
    }
}
pub const SET_SCAN_PARAMETERS_LEN: usize = 7;
impl Command for SetScanParameters {
    type Return = CommandComplete<StatusReturn>;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetScanParameters.into()
    }

    fn byte_len(&self) -> usize {
        SET_SCAN_PARAMETERS_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(SET_SCAN_PARAMETERS_LEN, buf)?;
        let scan_parameters = &self.0;
        let window = u16::from(scan_parameters.scan_window);
        let interval = u16::from(scan_parameters.scan_interval);
        if window > interval {
            // The scan window should always be less than or equal to the scan interval.
            return Err(PackError::InvalidFields);
        }
        buf[0] = scan_parameters.scan_type.into();
        buf[1..3].copy_from_slice(&interval.to_bytes_le()[..]);
        buf[3..5].copy_from_slice(&window.to_bytes_le()[..]);
        buf[5] = scan_parameters.own_address_type.into();
        buf[6] = scan_parameters.scanning_filter_policy.into();
        Ok(())
    }

    fn unpack_from(_buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
pub const MAX_RESPONSE_DATA_LEN: usize = 31;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct SetScanResponseData {
    len: u8,
    data: [u8; MAX_RESPONSE_DATA_LEN],
}
impl SetScanResponseData {
    pub const BYTE_LEN: usize = MAX_RESPONSE_DATA_LEN + 1;
    pub const OPCODE: LEControllerOpcode = LEControllerOpcode::SetScanResponseData;
    pub const ZEROED: SetScanResponseData = SetScanResponseData {
        len: 0,
        data: [0_u8; MAX_RESPONSE_DATA_LEN],
    };
    pub fn from_slice(data: &[u8]) -> Option<SetScanResponseData> {
        let data_len: u8 = data.as_ref().len().try_into().ok()?;
        if usize::from(data_len) > MAX_RESPONSE_DATA_LEN {
            None
        } else {
            let mut buf = [0_u8; MAX_RESPONSE_DATA_LEN];
            buf[..usize::from(data_len)].copy_from_slice(data);
            Some(SetScanResponseData {
                len: data_len,
                data: buf,
            })
        }
    }
}
impl<'a> TryFrom<&'a [u8]> for SetScanResponseData {
    type Error = PackError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let len = value.len();
        Self::from_slice(value).ok_or(PackError::BadLength {
            expected: MAX_RESPONSE_DATA_LEN,
            got: len,
        })
    }
}
impl AsRef<[u8]> for SetScanResponseData {
    fn as_ref(&self) -> &[u8] {
        &self.data[..usize::from(self.len)]
    }
}
impl AsMut<[u8]> for SetScanResponseData {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data[..usize::from(self.len)]
    }
}
impl Command for SetScanResponseData {
    type Return = CommandComplete<StatusReturn>;

    fn opcode() -> Opcode {
        Self::OPCODE.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.len;
        buf[1..].copy_from_slice(self.data.as_ref());
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        let len = buf[0];
        let mut data = [0_u8; MAX_RESPONSE_DATA_LEN];
        data.copy_from_slice(&buf[1..]);
        Ok(SetScanResponseData { len, data })
    }
}

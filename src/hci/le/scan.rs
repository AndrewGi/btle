//! LE [`SetScanEnable`], [`SetScanParameters`], and other primitive scan types.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, StatusReturn};
use crate::hci::le::LEControllerOpcode;
use crate::hci::Opcode;
use crate::le::scan::ScanParameters;
use crate::PackError;

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

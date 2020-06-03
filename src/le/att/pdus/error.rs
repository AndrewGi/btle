use crate::le::att::attribute::Handle;
use crate::le::att::error::Code;
use crate::le::att::pdus::{PackablePDU, UnpackablePDU};
use crate::le::att::Opcode;
use crate::PackError;
use core::convert::{TryFrom, TryInto};

pub struct ErrorRsp {
    pub opcode_in_error: Opcode,
    pub handle_in_error: Handle,
    pub error_code: Code,
}
impl ErrorRsp {
    pub const BYTE_LEN: usize = 1 + 2 + 1;
}
impl PackablePDU for ErrorRsp {
    const OPCODE: Opcode = Opcode::ErrorRsp;

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.opcode_in_error.into();
        buf[1..3].copy_from_slice(&self.handle_in_error.inner().to_le_bytes());
        buf[3] = self.error_code.into();
        Ok(())
    }
}
impl UnpackablePDU for ErrorRsp {
    fn unpack_from(buf: &[u8]) -> Result<Self, PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        let opcode_in_error = Opcode::try_from(buf[0]).map_err(|_| PackError::bad_index(0))?;
        let error_code = Code::try_from(buf[3]).map_err(|_| PackError::bad_index(3))?;
        let handle_in_error = Handle::new(u16::from_le_bytes(
            (&buf[1..3]).try_into().expect("len checked above"),
        ));
        Ok(ErrorRsp {
            opcode_in_error,
            handle_in_error,
            error_code,
        })
    }
}

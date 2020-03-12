//! LE [`Rand`] command and return parameters.
use crate::hci::command::Command;
use crate::hci::event::ReturnParameters;
use crate::hci::le::LEControllerOpcode;
use crate::hci::{ErrorCode, Opcode};
use crate::PackError;
use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Rand {}
impl Command for Rand {
    type Return = RandReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::Rand.into()
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
        Ok(Self {})
    }
}
pub const RAND_LEN: usize = 8;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct RandReturn {
    pub status: ErrorCode,
    pub random_bytes: [u8; RAND_LEN],
}
impl ReturnParameters for RandReturn {
    fn byte_len(&self) -> usize {
        RAND_LEN + 1
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(RAND_LEN + 1, buf)?;
        buf[0] = self.status.into();
        buf[1..RAND_LEN + 1].copy_from_slice(&self.random_bytes[..]);
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(RAND_LEN + 1, buf)?;
        Ok(RandReturn {
            status: ErrorCode::try_from(buf[0]).map_err(|_| PackError::bad_index(0))?,
            random_bytes: (&buf[1..1 + RAND_LEN])
                .try_into()
                .expect("length checked above"),
        })
    }
}

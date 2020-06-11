use crate::le::att::pdus::{PackablePDU, Response, UnpackablePDU};
use crate::le::att::Opcode;
use crate::le::connection::MTU;
use crate::PackError;
use std::convert::TryInto;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ExchangeMTURsp(pub MTU);
impl ExchangeMTURsp {
    pub const BYTE_LEN: usize = MTU::BYTE_LEN;
}
impl PackablePDU for ExchangeMTURsp {
    const OPCODE: Opcode = Opcode::ExchangeMTURsp;

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf.copy_from_slice(u16::from(self.0).to_le_bytes().as_ref());
        Ok(())
    }
}
impl UnpackablePDU for ExchangeMTURsp {
    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        let mtu = MTU::new_checked(u16::from_le_bytes(
            buf.try_into().expect("length checked above"),
        ))
        .ok_or(PackError::bad_index(0))?;
        Ok(ExchangeMTURsp(mtu))
    }
}
impl Response for ExchangeMTURsp {}

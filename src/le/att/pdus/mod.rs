use crate::le::att::Opcode;
use crate::PackError;

pub mod error;
pub mod exchange;
pub mod find;
pub mod handle;
pub mod read;
pub mod write;

pub trait PackablePDU {
    const OPCODE: Opcode;
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
}
pub trait UnpackablePDU: PackablePDU {
    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
}

pub trait Request {}
pub trait Response {}

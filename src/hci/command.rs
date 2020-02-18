use crate::hci::{Opcode, OPCODE_LEN, HCIPackError};
use crate::hci::event::ReturnParameters;
use core::convert::TryFrom;

pub struct CommandPacket<Storage: AsRef<[u8]>> {
    opcode: Opcode,
    parameters: Storage,
}
pub trait Command {
    type Return: ReturnParameters;
    fn opcode() -> Opcode;
    fn full_len(&self) -> usize {
        self.byte_len() + OPCODE_LEN + 1
    }
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
    fn pack_full(&self, buf: &mut [u8]) -> Result<usize, HCIPackError> {
        if buf.len() != self.full_len() {
            Err(HCIPackError::BadLength)
        } else {
            self.pack_into(&mut buf[3..])?;
            Self::opcode().pack(&mut buf[..OPCODE_LEN])?;
            buf[2] =
                u8::try_from(self.byte_len()).expect("commands can only have 0xFF parameter bytes");
            Ok(self.byte_len() + OPCODE_LEN)
        }
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
        where
            Self: Sized;
}
use crate::hci::event::ReturnParameters;
use crate::hci::stream::PacketType;
use crate::hci::{HCIPackError, Opcode, OPCODE_LEN};
use core::convert::TryFrom;

pub struct CommandPacket<Storage: AsRef<[u8]>> {
    opcode: Opcode,
    parameters: Storage,
}
pub trait Command {
    type Return: ReturnParameters;
    fn opcode() -> Opcode;
    fn full_len(&self) -> usize {
        self.byte_len() + OPCODE_LEN + 2
    }
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
    fn pack_full(&self, buf: &mut [u8]) -> Result<usize, HCIPackError> {
        let full = self.full_len();
        if buf.len() != full {
            Err(HCIPackError::BadLength)
        } else {
            self.pack_into(&mut buf[4..])?;
            Self::opcode().pack(&mut buf[1..1 + OPCODE_LEN])?;
            buf[3] =
                u8::try_from(self.byte_len()).expect("commands can only have 0xFF parameter bytes");
            buf[0] = PacketType::Command.into();
            Ok(full)
        }
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
}
use crate::hci::event::ReturnParameters;
use crate::hci::packet::PacketType;
use crate::hci::{HCIPackError, Opcode, OPCODE_LEN};
use core::convert::TryFrom;

pub struct CommandPacket<Storage: AsRef<[u8]>> {
    pub opcode: Opcode,
    pub parameters: Storage,
}
pub struct CommandHeader {
    pub opcode: Opcode,
    pub len: u8,
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
        let full = self.full_len();
        // Trim buf to correct length
        let buf = &mut buf[..full];
        HCIPackError::expect_length(full, buf)?;
        self.pack_into(&mut buf[3..full])?;
        Self::opcode().pack(&mut buf[..OPCODE_LEN])?;
        buf[2] = u8::try_from(self.byte_len())
            .ok()
            .ok_or(HCIPackError::InvalidFields)?;
        Ok(full)
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
    fn unpack_command_packet<S: AsRef<[u8]>>(
        packet: &CommandPacket<S>,
    ) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        if packet.opcode != Self::opcode() {
            Err(HCIPackError::BadOpcode)
        } else {
            Self::unpack_from(packet.parameters.as_ref())
        }
    }
    fn packet_byte_len(&self) -> usize {
        self.full_len() + 1
    }

    fn packet_pack_into(&self, buf: &mut [u8]) -> Result<usize, HCIPackError> {
        let len = self.pack_full(&mut buf[1..])?;
        buf[0] = PacketType::Command.into();
        Ok(len + 1)
    }
}

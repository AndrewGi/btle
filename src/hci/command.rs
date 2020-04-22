//! HCI Command and command utilities.
use crate::bytes::Storage;
use crate::hci::event::ReturnParameters;
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::{Opcode, OPCODE_LEN};
use crate::PackError;
use core::convert::TryFrom;

/// Raw HCI Command Packet. Stores command [`Opcode`] and `parameters` (byte buffer).
/// [`Opcode`]: crate::hci::Opcode;
pub struct CommandPacket<Buf> {
    pub opcode: Opcode,
    pub parameters: Buf,
}
impl<Buf: AsRef<[u8]>> CommandPacket<Buf> {
    pub fn as_ref(&self) -> CommandPacket<&[u8]> {
        CommandPacket {
            opcode: self.opcode,
            parameters: self.parameters.as_ref(),
        }
    }
    pub fn to_raw_packet<NewStorage: Storage<u8>>(&self) -> RawPacket<NewStorage> {
        let len = self.parameters.as_ref().len() + OPCODE_LEN;
        let mut buf = NewStorage::with_size(len);
        buf.as_mut()[OPCODE_LEN..].copy_from_slice(self.parameters.as_ref());
        self.opcode
            .pack(&mut buf.as_mut()[..OPCODE_LEN])
            .expect("given a hardcoded length buf");
        RawPacket {
            packet_type: PacketType::Command,
            buf,
        }
    }
}
pub struct CommandHeader {
    pub opcode: Opcode,
    pub len: u8,
}
/// HCI Command trait for structs that are HCI commands.
pub trait Command {
    type Return: ReturnParameters;
    fn opcode() -> Opcode;
    fn full_len(&self) -> usize {
        self.byte_len() + OPCODE_LEN + 1
    }
    fn byte_len(&self) -> usize;
    /// Pack the command parameters into a byte buffer.
    /// !! `buf.len() == Command.byte_len()` otherwise will return `PackError::BadLength` !!
    /// # Errors
    ///
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
    fn pack_full(&self, buf: &mut [u8]) -> Result<usize, PackError> {
        let full = self.full_len();
        // Trim buf to correct length
        let buf = &mut buf[..full];
        PackError::expect_length(full, buf)?;
        self.pack_into(&mut buf[3..full])?;
        Self::opcode().pack(&mut buf[..OPCODE_LEN])?;
        buf[2] = u8::try_from(self.byte_len())
            .ok()
            .ok_or(PackError::InvalidFields)?;
        Ok(full)
    }
    fn pack_command_packet<S: Storage<u8>>(&self) -> Result<CommandPacket<S>, PackError> {
        let len = self.byte_len();
        let mut buf = S::with_size(len);
        self.pack_into(buf.as_mut())?;
        Ok(CommandPacket {
            opcode: Self::opcode(),
            parameters: buf,
        })
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
    fn unpack_command_packet<S: AsRef<[u8]>>(packet: &CommandPacket<S>) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if packet.opcode != Self::opcode() {
            Err(PackError::BadOpcode)
        } else {
            Self::unpack_from(packet.parameters.as_ref())
        }
    }
    fn packet_byte_len(&self) -> usize {
        self.full_len() + 1
    }

    fn packet_pack_into(&self, buf: &mut [u8]) -> Result<usize, PackError> {
        let len = self.pack_full(&mut buf[1..])?;
        buf[0] = PacketType::Command.into();
        Ok(len + 1)
    }
}

//! HCI Command and command utilities.
use crate::bytes::Storage;
use crate::hci::event::{Event, EventPacket, ReturnEvent};
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::{Opcode, OPCODE_LEN};
use crate::PackError;
use core::convert::TryFrom;
use core::convert::TryInto;

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
        let para_len = self.parameters.as_ref().len();
        let len = para_len + 1 + OPCODE_LEN;
        let mut buf = NewStorage::with_size(len);
        buf.as_mut()[OPCODE_LEN + 1..].copy_from_slice(self.parameters.as_ref());
        self.opcode
            .pack(&mut buf.as_mut()[..OPCODE_LEN])
            .expect("given a hardcoded length buf");
        buf.as_mut()[2] = para_len.try_into().expect("len bigger than an u8");
        RawPacket {
            packet_type: PacketType::Command,
            buf,
        }
    }
    pub fn pack_as_raw_packet<NewStorage: Storage<u8>>(&self) -> NewStorage {
        let para_len = self.parameters.as_ref().len();
        let len = para_len + OPCODE_LEN + 1 + 1;
        let mut out = NewStorage::with_size(len);
        out.as_mut()[0] = PacketType::Command.into();
        self.opcode
            .pack(&mut out.as_mut()[1..1 + OPCODE_LEN])
            .expect("given a hardcoded length buf");
        out.as_mut()[1 + OPCODE_LEN] = para_len.try_into().expect("len bigger than an u8");
        out.as_mut()[1 + 1 + OPCODE_LEN..].copy_from_slice(self.parameters.as_ref());
        out
    }
}
impl<Storage: AsRef<[u8]>> core::fmt::Debug for CommandPacket<Storage> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CommandPacket<Storage>")
            .field("opcode", &self.opcode)
            .field("parameters", &AsRef::<[u8]>::as_ref(&self.parameters))
            .finish()
    }
}
pub struct CommandHeader {
    pub opcode: Opcode,
    pub len: u8,
}
/// HCI Command trait for structs that are HCI commands.
pub trait Command {
    type Return: ReturnEvent;
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
    fn unpack_return(event: EventPacket<&[u8]>) -> Result<Option<Self::Return>, PackError> {
        if event.event_code() == Self::Return::EVENT_CODE {
            if let Some(guess) = Self::Return::guess_command_opcode(event.parameters()) {
                if Self::opcode() == guess {
                    return Ok(Some(Self::Return::event_unpack_from(event.parameters())?));
                }
            }
        }
        Ok(None)
    }
}

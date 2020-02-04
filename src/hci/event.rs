use crate::hci::{ErrorCode, Event, EventCode, HCIPackError, Opcode, OPCODE_LEN};
use core::convert::TryFrom;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct CommandComplete {
    pub status: ErrorCode,
    pub num_command_packets: u8,
    pub opcode: Opcode,
}
pub const COMMAND_COMPLETE_LEN: usize = 1 + 1 + OPCODE_LEN;
impl Event for CommandComplete {
    const CODE: EventCode = EventCode::CommandComplete;

    fn byte_len(&self) -> usize {
        COMMAND_COMPLETE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        if buf.len() < COMMAND_COMPLETE_LEN {
            Err(HCIPackError::SmallBuffer)
        } else {
            self.opcode.pack(&mut buf[2..4])?;
            buf[0] = self.status.into();
            buf[1] = self.num_command_packets;
            Ok(())
        }
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        if buf.len() != COMMAND_COMPLETE_LEN {
            Err(HCIPackError::BadLength)
        } else {
            let opcode = Opcode::unpack(&buf[2..4])?;
            let status = ErrorCode::try_from(buf[0]).map_err(|_| HCIPackError::BadBytes)?;
            Ok(CommandComplete {
                status,
                num_command_packets: buf[1],
                opcode,
            })
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct CommandStatus {
    pub status: ErrorCode,
    pub num_command_packets: u8,
    pub opcode: Opcode,
}
pub const COMMAND_STATUS_LEN: usize = 1 + 1 + OPCODE_LEN;
impl Event for CommandStatus {
    const CODE: EventCode = EventCode::CommandStatus;

    fn byte_len(&self) -> usize {
        COMMAND_STATUS_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        if buf.len() < COMMAND_STATUS_LEN {
            Err(HCIPackError::SmallBuffer)
        } else {
            self.opcode.pack(&mut buf[2..4])?;
            buf[0] = self.status.into();
            buf[1] = self.num_command_packets;
            Ok(())
        }
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        if buf.len() != COMMAND_STATUS_LEN {
            Err(HCIPackError::BadLength)
        } else {
            let opcode = Opcode::unpack(&buf[2..4])?;
            let status = ErrorCode::try_from(buf[0]).map_err(|_| HCIPackError::BadBytes)?;
            Ok(CommandStatus {
                status,
                num_command_packets: buf[1],
                opcode,
            })
        }
    }
}

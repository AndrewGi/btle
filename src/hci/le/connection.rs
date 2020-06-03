use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, ReturnParameters};
use crate::hci::le::LEControllerOpcode;
use crate::hci::{ErrorCode, Opcode};
use crate::PackError;
use core::convert::{TryFrom, TryInto};
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct ReadBufferSizeV1();
impl ReadBufferSizeV1 {
    pub const OPCODE: LEControllerOpcode = LEControllerOpcode::ReadBufferSizeV1;
}
pub struct BufferSizeV1 {
    pub status: ErrorCode,
    pub le_acl_data_packet_len: u16,
    pub total_num_le_acl_data_packets: u8,
}
impl BufferSizeV1 {
    pub const BYTE_LEN: usize = 1 + 2 + 1;
}
impl Command for ReadBufferSizeV1 {
    type Return = CommandComplete<BufferSizeV1>;

    fn opcode() -> Opcode {
        Self::OPCODE.into()
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
        Ok(ReadBufferSizeV1())
    }
}
impl ReturnParameters for BufferSizeV1 {
    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.status.into();
        buf[1..3].copy_from_slice(&self.le_acl_data_packet_len.to_le_bytes()[..]);
        buf[3] = self.total_num_le_acl_data_packets;
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        let status = ErrorCode::try_from(buf[0]).map_err(|_| PackError::bad_index(0))?;
        let le_acl_data_packet_len =
            u16::from_le_bytes((&buf[1..3]).try_into().expect("len checked above"));
        let total_num_le_acl_data_packets = buf[3];
        Ok(BufferSizeV1 {
            status,
            le_acl_data_packet_len,
            total_num_le_acl_data_packets,
        })
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct ReadBufferSizeV2();

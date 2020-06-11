use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, CommandStatus, ReturnParameters};
use crate::hci::le::{LEControllerOpcode, MetaEventCode};
use crate::hci::{ErrorCode, Opcode};
use crate::le::advertiser::PeerAddressType;
use crate::le::connection::{
    CELength, ConnectionHandle, ConnectionInterval, ConnectionLatency, InitiatorFilterPolicy,
    MasterClockAccuracy, Role, SupervisionTimeout,
};
use crate::le::scan::{OwnAddressType, ScanInterval, ScanWindow};
use crate::{BTAddress, PackError, BT_ADDRESS_LEN};
use core::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ReadBufferSizeV1();
impl ReadBufferSizeV1 {
    pub const OPCODE: LEControllerOpcode = LEControllerOpcode::ReadBufferSizeV1;
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
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
impl ReadBufferSizeV2 {
    pub const OPCODE: LEControllerOpcode = LEControllerOpcode::ReadBufferSizeV2;
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct BufferSizeV2 {
    pub status: ErrorCode,
    pub le_acl_data_packet_len: u16,
    pub total_num_le_acl_data_packets: u8,
    pub iso_data_packet_len: u16,
    pub total_num_iso_data_packets: u8,
}
impl BufferSizeV2 {
    pub const BYTE_LEN: usize = 1 + 2 + 1 + 2 + 1;
}

impl Command for ReadBufferSizeV2 {
    type Return = CommandComplete<BufferSizeV2>;

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
        Ok(ReadBufferSizeV2())
    }
}
impl ReturnParameters for BufferSizeV2 {
    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.status.into();
        buf[1..3].copy_from_slice(&self.le_acl_data_packet_len.to_le_bytes()[..]);
        buf[3] = self.total_num_le_acl_data_packets;
        buf[4..6].copy_from_slice(&self.iso_data_packet_len.to_le_bytes()[..]);
        buf[6] = self.total_num_iso_data_packets;
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
        let iso_data_packet_len =
            u16::from_le_bytes((&buf[4..6]).try_into().expect("len checked above"));
        let total_num_iso_data_packets = buf[6];
        Ok(BufferSizeV2 {
            status,
            le_acl_data_packet_len,
            total_num_le_acl_data_packets,
            iso_data_packet_len,
            total_num_iso_data_packets,
        })
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct CreateConnection {
    pub le_scan_interval: ScanInterval,
    pub le_scan_window: ScanWindow,
    pub initiator_filter_policy: InitiatorFilterPolicy,
    pub peer_address_type: PeerAddressType,
    pub peer_address: BTAddress,
    pub own_address_type: OwnAddressType,
    pub connection_interval_min: ConnectionInterval,
    pub connection_interval_max: ConnectionInterval,
    pub connection_latency: ConnectionLatency,
    pub supervision_timeout: SupervisionTimeout,
    pub min_ce_len: CELength,
    pub max_ce_len: CELength,
}
impl CreateConnection {
    pub const OPCODE: LEControllerOpcode = LEControllerOpcode::CreateConnection;
    pub const BYTE_LEN: usize = ScanInterval::BYTE_LEN
        + ScanWindow::BYTE_LEN
        + InitiatorFilterPolicy::BYTE_LEN
        + PeerAddressType::BYTE_LEN
        + BT_ADDRESS_LEN
        + ScanInterval::BYTE_LEN * 2
        + ConnectionLatency::BYTE_LEN
        + SupervisionTimeout::BYTE_LEN
        + CELength::BYTE_LEN * 2;
}
impl Command for CreateConnection {
    type Return = CommandStatus;

    fn opcode() -> Opcode {
        Self::OPCODE.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0..2].copy_from_slice(u16::from(self.le_scan_interval).to_le_bytes().as_ref());
        buf[2..4].copy_from_slice(u16::from(self.le_scan_window).to_le_bytes().as_ref());
        buf[4] = self.initiator_filter_policy.into();
        buf[5] = self.peer_address_type.into();
        buf[6..12].copy_from_slice(self.peer_address.0.as_ref());
        buf[12] = self.own_address_type.into();
        buf[13..15].copy_from_slice(
            u16::from(self.connection_interval_min)
                .to_le_bytes()
                .as_ref(),
        );
        buf[15..17].copy_from_slice(
            u16::from(self.connection_interval_max)
                .to_le_bytes()
                .as_ref(),
        );
        buf[17..19].copy_from_slice(u16::from(self.connection_latency).to_le_bytes().as_ref());
        buf[19..21].copy_from_slice(u16::from(self.supervision_timeout).to_le_bytes().as_ref());
        buf[21..23].copy_from_slice(u16::from(self.min_ce_len).to_le_bytes().as_ref());
        buf[23..25].copy_from_slice(u16::from(self.max_ce_len).to_le_bytes().as_ref());
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        todo!("implement unpack from for CreateConnection")
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ConnectionCompleteEvent {
    pub status: ErrorCode,
    pub connection_handle: ConnectionHandle,
    pub role: Role,
    pub peer_address_type: PeerAddressType,
    pub peer_address: BTAddress,
    pub connection_interval: ConnectionInterval,
    pub connection_latency: ConnectionLatency,
    pub supervision_timeout: SupervisionTimeout,
    pub master_clock_accuracy: MasterClockAccuracy,
}
impl ConnectionCompleteEvent {
    pub const CODE: MetaEventCode = MetaEventCode::ConnectionComplete;
    pub const BYTE_LEN: usize = ErrorCode::BYTE_LEN
        + ConnectionHandle::BYTE_LEN
        + Role::BYTE_LEN
        + PeerAddressType::BYTE_LEN
        + BT_ADDRESS_LEN
        + ConnectionInterval::BYTE_LEN
        + ConnectionLatency::BYTE_LEN
        + SupervisionTimeout::BYTE_LEN
        + MasterClockAccuracy::BYTE_LEN;
}

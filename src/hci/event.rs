use crate::hci::stream::PacketType;
use crate::hci::{ErrorCode, HCIConversionError, HCIPackError, Opcode, EVENT_CODE_LEN, OPCODE_LEN};
use core::convert::TryFrom;

/// HCI Event Code. 8-bit code corresponding to an HCI Event. Check the Bluetooth Core Spec for more.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum EventCode {
    InquiryComplete = 0x01,
    InquiryResult = 0x02,
    ConnectionComplete = 0x03,
    ConnectionRequest = 0x04,
    DisconnectionComplete = 0x05,
    AuthenticationComplete = 0x06,
    RemoteNameRequestComplete = 0x07,
    EncryptionChange = 0x08,
    ChangeConnectionLinkKeyComplete = 0x09,
    MasterLinkKeyComplete = 0x0A,
    ReadRemoteSupportedFeaturesComplete = 0x0B,
    ReadRemoteVersionInformationComplete = 0x0C,
    QoSSetupComplete = 0x0D,
    CommandComplete = 0x0E,
    CommandStatus = 0x0F,
    FlushOccurred = 0x11,
    RoleChange = 0x12,
    NumberOfCompletedPackets = 0x13,
    ModeChange = 0x14,
    ReturnLinkKeys = 0x15,
    PINCodeRequest = 0x16,
    LinkKeyRequest = 0x17,
    LinkKeyNotification = 0x18,
    LoopbackCommand = 0x19,
    DataBufferOverflow = 0x1A,
    MaxSlotsChange = 0x1B,
    ReadClockOffsetComplete = 0x1C,
    ConnectionPacketTypeChanged = 0x1D,
    QoSViolation = 0x1E,
    PageScanRepetitionModeChange = 0x20,
    FlowSpecificationComplete = 0x21,
    InquiryResultWithRSSI = 0x22,
    ReadRemoteExtendedFeaturesComplete = 0x23,
    SynchronousConnectionComplete = 0x2C,
    SynchronousConnectionChanged = 0x2D,
    SniffSubrating = 0x2E,
    ExtendedInquiryResult = 0x2F,
    EncryptionKeyRefreshComplete = 0x30,
    IOCapabilityRequest = 0x31,
    IOCapabilityResponse = 0x32,
    UserConfirmationRequest = 0x33,
    UserPasskeyRequest = 0x34,
    RemoteOOBDataRequest = 0x35,
    SimplePairingComplete = 0x36,
    LinkSupervisionTimeoutChanged = 0x38,
    EnhancedFlushComplete = 0x39,
    UserPasskeyNotification = 0x3B,
    KeypressNotification = 0x3C,
    RemoteHostSupportedFeaturesNotification = 0x3D,
    PhysicalLinkComplete = 0x40,
    ChannelSelected = 0x41,
    DisconnectionPhysicalLinkComplete = 0x42,
    PhysicalLinkLostEarlyWarning = 0x43,
    PhysicalLinkRecovery = 0x44,
    LogicalLinkComplete = 0x45,
    DisconnectionLogicalLinkComplete = 0x46,
    FlowSpecModifyComplete = 0x47,
    NumberOfCompletedDataBlocks = 0x48,
    ShortRangeModeChangeComplete = 0x4C,
    AMPStatusChange = 0x4D,
    AMPStartTest = 0x49,
    AMPTestEnd = 0x4A,
    AMPReceiverReport = 0x4B,
    LEMeta = 0x3E,
}
impl From<EventCode> for u8 {
    fn from(code: EventCode) -> Self {
        code as u8
    }
}
impl From<EventCode> for u32 {
    fn from(code: EventCode) -> Self {
        code as u32
    }
}
impl TryFrom<u8> for EventCode {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(EventCode::InquiryComplete),
            0x02 => Ok(EventCode::InquiryResult),
            0x03 => Ok(EventCode::ConnectionComplete),
            0x04 => Ok(EventCode::ConnectionRequest),
            0x05 => Ok(EventCode::DisconnectionComplete),
            0x06 => Ok(EventCode::AuthenticationComplete),
            0x07 => Ok(EventCode::RemoteNameRequestComplete),
            0x08 => Ok(EventCode::EncryptionChange),
            0x09 => Ok(EventCode::ChangeConnectionLinkKeyComplete),
            0x0A => Ok(EventCode::MasterLinkKeyComplete),
            0x0B => Ok(EventCode::ReadRemoteSupportedFeaturesComplete),
            0x0C => Ok(EventCode::ReadRemoteVersionInformationComplete),
            0x0D => Ok(EventCode::QoSSetupComplete),
            0x0E => Ok(EventCode::CommandComplete),
            0x0F => Ok(EventCode::CommandStatus),
            0x11 => Ok(EventCode::FlushOccurred),
            0x12 => Ok(EventCode::RoleChange),
            0x13 => Ok(EventCode::NumberOfCompletedPackets),
            0x14 => Ok(EventCode::ModeChange),
            0x15 => Ok(EventCode::ReturnLinkKeys),
            0x16 => Ok(EventCode::PINCodeRequest),
            0x17 => Ok(EventCode::LinkKeyRequest),
            0x18 => Ok(EventCode::LinkKeyNotification),
            0x19 => Ok(EventCode::LoopbackCommand),
            0x1A => Ok(EventCode::DataBufferOverflow),
            0x1B => Ok(EventCode::MaxSlotsChange),
            0x1C => Ok(EventCode::ReadClockOffsetComplete),
            0x1D => Ok(EventCode::ConnectionPacketTypeChanged),
            0x1E => Ok(EventCode::QoSViolation),
            0x20 => Ok(EventCode::PageScanRepetitionModeChange),
            0x21 => Ok(EventCode::FlowSpecificationComplete),
            0x22 => Ok(EventCode::InquiryResultWithRSSI),
            0x23 => Ok(EventCode::ReadRemoteExtendedFeaturesComplete),
            0x2C => Ok(EventCode::SynchronousConnectionComplete),
            0x2D => Ok(EventCode::SynchronousConnectionChanged),
            0x2E => Ok(EventCode::SniffSubrating),
            0x2F => Ok(EventCode::ExtendedInquiryResult),
            0x30 => Ok(EventCode::EncryptionKeyRefreshComplete),
            0x33 => Ok(EventCode::IOCapabilityRequest),
            0x32 => Ok(EventCode::IOCapabilityResponse),
            0x31 => Ok(EventCode::UserConfirmationRequest),
            0x34 => Ok(EventCode::UserPasskeyRequest),
            0x35 => Ok(EventCode::RemoteOOBDataRequest),
            0x36 => Ok(EventCode::SimplePairingComplete),
            0x38 => Ok(EventCode::LinkSupervisionTimeoutChanged),
            0x39 => Ok(EventCode::EnhancedFlushComplete),
            0x3B => Ok(EventCode::UserPasskeyNotification),
            0x3C => Ok(EventCode::KeypressNotification),
            0x3D => Ok(EventCode::RemoteHostSupportedFeaturesNotification),
            0x40 => Ok(EventCode::PhysicalLinkComplete),
            0x41 => Ok(EventCode::ChannelSelected),
            0x42 => Ok(EventCode::DisconnectionPhysicalLinkComplete),
            0x43 => Ok(EventCode::PhysicalLinkLostEarlyWarning),
            0x44 => Ok(EventCode::PhysicalLinkRecovery),
            0x45 => Ok(EventCode::LogicalLinkComplete),
            0x46 => Ok(EventCode::DisconnectionLogicalLinkComplete),
            0x47 => Ok(EventCode::FlowSpecModifyComplete),
            0x48 => Ok(EventCode::NumberOfCompletedDataBlocks),
            0x4C => Ok(EventCode::ShortRangeModeChangeComplete),
            0x4D => Ok(EventCode::AMPStatusChange),
            0x49 => Ok(EventCode::AMPStartTest),
            0x4A => Ok(EventCode::AMPTestEnd),
            0x4B => Ok(EventCode::AMPReceiverReport),
            0x3E => Ok(EventCode::LEMeta),
            _ => Err(HCIConversionError(())),
        }
    }
}
pub trait Event {
    const CODE: EventCode;
    fn byte_len(&self) -> usize;
    fn full_len(&self) -> usize {
        self.byte_len() + EVENT_CODE_LEN + 2
    }
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
    fn pack_full(&self, buf: &mut [u8]) -> Result<usize, HCIPackError> {
        let full = self.full_len();
        if buf.len() != full {
            Err(HCIPackError::BadLength)
        } else {
            self.pack_into(&mut buf[3..])?;
            buf[0] = PacketType::Event.into();
            buf[1] = Self::CODE.into();
            buf[2] =
                u8::try_from(self.byte_len()).expect("events can only have 0xFF parameter bytes");
            Ok(full)
        }
    }
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
}

/// Unprocessed HCI Event Packet
pub struct EventPacket<Storage: AsRef<[u8]>> {
    event_opcode: EventCode,
    parameters: Storage,
}
impl<Storage: AsRef<[u8]>> EventPacket<Storage> {
    pub fn new(opcode: EventCode, parameters: Storage) -> Self {
        Self {
            event_opcode: opcode,
            parameters,
        }
    }
    pub fn event_code(&self) -> EventCode {
        self.event_opcode
    }
    pub fn parameters(&self) -> &[u8] {
        self.parameters.as_ref()
    }
    pub fn take_parameters(self) -> Storage {
        self.parameters
    }
}

pub trait ReturnParameters {
    const EVENT_CODE: EventCode;
    fn byte_len(&self) -> usize;
    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError>;
}

pub struct StatusReturn {
    pub status: ErrorCode,
}
impl StatusReturn {
    pub const fn byte_len() -> usize {
        1
    }
}
impl ReturnParameters for StatusReturn {
    const EVENT_CODE: EventCode = EventCode::CommandStatus;

    fn byte_len(&self) -> usize {
        Self::byte_len()
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError> {
        if buf.len() != Self::byte_len() {
            Err(HCIPackError::BadLength)
        } else {
            Ok(Self {
                status: ErrorCode::try_from(buf[0]).map_err(|_| HCIPackError::BadBytes)?,
            })
        }
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        if buf.len() != Self::byte_len() {
            Err(HCIPackError::BadLength)
        } else {
            buf[0] = self.status.into();
            Ok(())
        }
    }
}

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

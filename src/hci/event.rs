//! HCI Event and event utilities.
use crate::bytes::{StaticBuf, Storage};
use crate::hci::packet::{PacketType, RawPacket};
use crate::hci::{ErrorCode, Opcode, EVENT_CODE_LEN, OPCODE_LEN};
use crate::ConversionError;
use crate::PackError;
use core::convert::{TryFrom, TryInto};
use core::fmt::Formatter;

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
    type Error = ConversionError;

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
            _ => Err(ConversionError(())),
        }
    }
}
pub trait Event {
    const EVENT_CODE: EventCode;
    /// The byte length of the `Event` parameters.
    fn event_byte_len(&self) -> usize;
    fn event_full_byte_len(&self) -> usize {
        self.event_byte_len() + EVENT_CODE_LEN + 1
    }
    /// Unpack the `Event` from a byte buffer.
    fn event_unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
    fn unpack_event_packet<S: AsRef<[u8]>>(packet: &EventPacket<S>) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if packet.event_code != Self::EVENT_CODE {
            Err(PackError::BadOpcode)
        } else {
            Self::event_unpack_from(packet.parameters())
        }
    }
    /// Pack the `Event` parameters into a byte buffer.
    fn event_pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
    fn event_pack_packet<S: Storage<u8>>(&self) -> Result<EventPacket<S>, PackError> {
        let mut out = S::with_size(self.event_byte_len());
        self.event_pack_into(out.as_mut())?;
        Ok(EventPacket {
            event_code: Self::EVENT_CODE,
            parameters: out,
        })
    }
}
pub const MAX_HCI_PACKET_SIZE: usize = 255 + 2 + 1 + 1;
#[derive(Copy, Clone)]
pub struct FullHCIBuffer(pub [u8; MAX_HCI_PACKET_SIZE]);
impl FullHCIBuffer {
    pub const DEFAULT: FullHCIBuffer = FullHCIBuffer([0_u8; MAX_HCI_PACKET_SIZE]);
}
impl Default for FullHCIBuffer {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl AsRef<[u8]> for FullHCIBuffer {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl AsMut<[u8]> for FullHCIBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}
impl From<FullHCIBuffer> for [u8; MAX_HCI_PACKET_SIZE] {
    fn from(b: FullHCIBuffer) -> Self {
        b.0
    }
}
pub type StaticHCIBuffer = StaticBuf<u8, FullHCIBuffer>;
/// Unprocessed HCI Event Packet
pub struct EventPacket<Storage> {
    pub event_code: EventCode,
    pub parameters: Storage,
}
impl<Storage: AsRef<[u8]>> EventPacket<Storage> {
    pub fn new(opcode: EventCode, parameters: Storage) -> Self {
        Self {
            event_code: opcode,
            parameters,
        }
    }
    pub fn as_ref(&self) -> EventPacket<&'_ [u8]> {
        EventPacket {
            event_code: self.event_code,
            parameters: self.parameters.as_ref(),
        }
    }
    pub fn event_code(&self) -> EventCode {
        self.event_code
    }
    pub fn parameters(&self) -> &[u8] {
        self.parameters.as_ref()
    }
    pub fn take_parameters(self) -> Storage {
        self.parameters
    }
    pub fn to_raw_packet<NewStorage: crate::bytes::Storage<u8>>(&self) -> RawPacket<NewStorage> {
        let para_len = self.parameters.as_ref().len();
        let len = para_len + 1 + 1;
        let mut buf = NewStorage::with_size(len);
        buf.as_mut()[2..].copy_from_slice(self.parameters.as_ref());
        buf.as_mut()[0] = self.event_code.into();
        buf.as_mut()[1] = para_len.try_into().expect("len bigger than an u8");
        RawPacket {
            packet_type: PacketType::Event,
            buf,
        }
    }
    pub fn to_new_storage<NewStorage: crate::bytes::Storage<u8>>(&self) -> EventPacket<NewStorage> {
        EventPacket {
            event_code: self.event_code,
            parameters: NewStorage::from_slice(self.parameters().as_ref()),
        }
    }
}
impl<'a> TryFrom<RawPacket<&'a [u8]>> for EventPacket<&'a [u8]> {
    type Error = PackError;

    fn try_from(packet: RawPacket<&'a [u8]>) -> Result<Self, Self::Error> {
        if packet.packet_type != PacketType::Event {
            Err(PackError::BadOpcode)
        } else {
            let code = match packet.buf.get(0) {
                None => {
                    return Err(PackError::BadLength {
                        expected: 2,
                        got: 0,
                    })
                }
                Some(&b) => EventCode::try_from(b).ok().ok_or(PackError::bad_index(0))?,
            };
            let len = match packet.buf.get(1) {
                None => {
                    return Err(PackError::BadLength {
                        expected: 2,
                        got: 1,
                    })
                }
                Some(l) => *l,
            };
            if usize::from(len) != (packet.buf.len() - 2) {
                // Packet length is incorrect
                Err(PackError::InvalidFields)
            } else {
                Ok(EventPacket::new(code, &packet.buf[2..]))
            }
        }
    }
}
impl<Storage: AsRef<[u8]>> core::fmt::Debug for EventPacket<Storage> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        f.debug_struct("EventPacket")
            .field("event_code", &self.event_code)
            .field("parameters", &AsRef::<[u8]>::as_ref(&self.parameters))
            .finish()
    }
}
pub trait ReturnParameters {
    fn byte_len(&self) -> usize;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
}
pub struct StatusReturn {
    pub status: ErrorCode,
}
impl ReturnParameters for StatusReturn {
    fn byte_len(&self) -> usize {
        1
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(1, buf)?;
        buf[0] = self.status.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(1, buf)?;
        Ok(StatusReturn {
            status: ErrorCode::try_from(buf[0]).map_err(|_| PackError::bad_index(0))?,
        })
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct CommandComplete<Params: ReturnParameters> {
    pub num_command_packets: u8,
    pub opcode: Opcode,
    pub params: Params,
}
impl<Params: ReturnParameters> CommandComplete<Params> {}
pub const COMMAND_COMPLETE_HEADER_LEN: usize = 1 + OPCODE_LEN;
impl<Params: ReturnParameters> Event for CommandComplete<Params> {
    const EVENT_CODE: EventCode = EventCode::CommandComplete;

    fn event_byte_len(&self) -> usize {
        COMMAND_COMPLETE_HEADER_LEN + self.params.byte_len()
    }

    fn event_unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if buf.len() < COMMAND_COMPLETE_HEADER_LEN {
            Err(PackError::BadLength {
                expected: COMMAND_COMPLETE_HEADER_LEN,
                got: buf.len(),
            })
        } else {
            let opcode = Opcode::unpack(&buf[1..3])?;
            Ok(CommandComplete {
                num_command_packets: buf[0],
                opcode,
                params: Params::unpack_from(&buf[3..])?,
            })
        }
    }

    fn event_pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.event_byte_len(), buf)?;
        self.params.pack_into(&mut buf[3..])?;
        self.opcode.pack(&mut buf[1..3])?;
        buf[0] = self.num_command_packets;
        Ok(())
    }
}
pub trait ReturnEvent: Event {
    fn command_opcode(&self) -> Opcode;
    fn guess_command_opcode(buf: &[u8]) -> Option<Opcode>;
}
impl<Params: ReturnParameters> ReturnEvent for CommandComplete<Params> {
    fn command_opcode(&self) -> Opcode {
        self.opcode
    }
    fn guess_command_opcode(buf: &[u8]) -> Option<Opcode> {
        if buf.len() >= 3 {
            Opcode::unpack(&buf[1..3]).ok()
        } else {
            None
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
    const EVENT_CODE: EventCode = EventCode::CommandStatus;

    fn event_byte_len(&self) -> usize {
        COMMAND_STATUS_LEN
    }

    fn event_unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(COMMAND_STATUS_LEN, buf)?;
        let opcode = Opcode::unpack(&buf[2..4])?;
        let status =
            ErrorCode::try_from(buf[0]).map_err(|_| PackError::BadBytes { index: Some(0) })?;
        Ok(CommandStatus {
            status,
            num_command_packets: buf[1],
            opcode,
        })
    }

    fn event_pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(COMMAND_STATUS_LEN, buf)?;
        self.opcode.pack(&mut buf[2..4])?;
        buf[0] = self.status.into();
        buf[1] = self.num_command_packets;
        Ok(())
    }
}

impl ReturnEvent for CommandStatus {
    fn command_opcode(&self) -> Opcode {
        self.opcode
    }
    fn guess_command_opcode(buf: &[u8]) -> Option<Opcode> {
        if buf.len() >= COMMAND_STATUS_LEN {
            Opcode::unpack(&buf[2..4]).ok()
        } else {
            None
        }
    }
}

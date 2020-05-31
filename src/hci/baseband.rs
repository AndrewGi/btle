use crate::hci::command::Command;
use crate::hci::event::{CommandComplete, StatusReturn};
use crate::hci::{Opcode, OCF, OGF};
use crate::PackError;
use core::convert::TryInto;

pub enum ControllerBasebandOpcode {
    SetEventMask = 0x0001,
    Reset = 0x0003,
    SetEventFilter = 0x0005,
    Flush = 0x0008,
    ReadPIN = 0x0009,
    WritePIN = 0x000A,
    ReadStoredLinkKey = 0x000D,
}
impl From<ControllerBasebandOpcode> for u16 {
    fn from(opcode: ControllerBasebandOpcode) -> Self {
        opcode as u16
    }
}
impl From<ControllerBasebandOpcode> for OCF {
    fn from(opcode: ControllerBasebandOpcode) -> Self {
        OCF::new(opcode.into())
    }
}
impl From<ControllerBasebandOpcode> for Opcode {
    fn from(ocf: ControllerBasebandOpcode) -> Self {
        Opcode(OGF::HCIControlBaseband, ocf.into())
    }
}
pub struct Reset;
impl Reset {
    pub const OPCODE: ControllerBasebandOpcode = ControllerBasebandOpcode::Reset;
}
impl Command for Reset {
    type Return = CommandComplete<StatusReturn>;

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
        Ok(Reset)
    }
}
pub enum EventMaskFlags {
    InquiryComplete = 0x00,
    InquiryResult = 0x01,
    ConnectionComplete = 0x02,
    ConnectionRequest = 0x03,
    DisconnectionComplete = 0x04,
    AuthenticationComplete = 0x05,
    RemoteNameRequestComplete = 0x06,
    EncryptionChange = 0x07,
    ChangeConnectionLinkKeyComplete = 0x08,
    MasterLinkKeyComplete = 0x09,
    ReadRemoteSupportedFeaturesComplete = 0x0A,
    ReadRemoteVersionInformationComplete = 0x0B,
    QoSSetupComplete = 0x0C,
    // 13, 14 are skipped
    HardwareError = 0x0F,
    FlushOccurred = 0x10,
    RoleChanged = 0x11,
    // 18 skipped
    ModeChange = 0x13,
    ReturnLinkKey = 0x14,
    PinCodeRequest = 0x15,
    LinkKeyRequest = 0x16,
    LinkKeyNotification = 0x17,
    LoopbackCommand = 0x18,
    DataBufferOverflow = 0x19,
    MaxSlotsChanged = 0x1A,
    ReadClockOffsetComplete = 0x1B,
    ConnectionPacketTypeChanged = 0x1C,
    QoSViolation = 0x1D,
    PageScanModeChange = 0x1E, // Deprecated
    PageScanRepetitionModeChange = 0x1F,
    FlowSpecificationComplete = 0x20,
    InquiryResultWithRSSI = 0x21,
    ReadRemoteExtendedFeaturesComplete = 0x22,
    // 35-42 Skipped
    SynchronousConnectionCompleted = 0x2B,
    SynchronousConnectionChanged = 0x2C,
    SniffSubrating = 0x2D,
    ExtendedInquiryResult = 0x2E,
    EncryptionKeyRefreshComplete = 0x2F,
    IOCapabilityRequest = 0x30,
    IOCapabilityResponse = 0x31,
    UserConfirmationRequest = 0x32,
    UserPasskeyRequest = 0x33,
    RemoteOOBDataRequest = 0x34,
    SimplePairingComplete = 0x35,
    // 54 skipped
    LinkSupervisionTimeoutChanged = 0x37,
    EnhancedFlushComplete = 0x38,
    // 57 skipped
    UserPasskeyNotification = 0x3A,
    KeypressNotification = 0x3B,
    RemoteHouseSupportedFeaturesNotification = 0x3C,
    LEMetaEvent = 0x3D,
}
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq)]
pub struct EventMask(pub u64);
impl From<EventMaskFlags> for u8 {
    fn from(f: EventMaskFlags) -> Self {
        f as u8
    }
}
impl From<EventMaskFlags> for u64 {
    fn from(f: EventMaskFlags) -> Self {
        f as u64
    }
}
impl EventMask {
    pub const DEFAULT: EventMask = EventMask(0x0000_1FFF_FFFF_FFFF);
    pub const ZEROED: EventMask = EventMask(0);
    pub const fn zeroed() -> EventMask {
        EventMask::ZEROED
    }
    pub fn enable_event(&mut self, flag: EventMaskFlags) {
        self.0 |= 1 << u64::from(flag);
    }
    pub fn disable_event(&mut self, flag: EventMaskFlags) {
        self.0 &= !(1 << u64::from(flag));
    }
    pub fn get_event(&mut self, flag: EventMaskFlags) -> bool {
        self.0 & (1 << u64::from(flag)) != 0
    }
}
impl Default for EventMask {
    fn default() -> Self {
        EventMask::DEFAULT
    }
}
#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq, Default)]
pub struct SetEventMask(pub EventMask);

impl SetEventMask {
    pub const BYTE_LEN: usize = 8;
    pub const OPCODE: ControllerBasebandOpcode = ControllerBasebandOpcode::SetEventMask;
}
impl Command for SetEventMask {
    type Return = CommandComplete<StatusReturn>;

    fn opcode() -> Opcode {
        Self::OPCODE.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf.copy_from_slice((self.0).0.to_le_bytes().as_ref());
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        Ok(SetEventMask(EventMask(u64::from_le_bytes(
            buf.try_into().map_err(|_| PackError::BadLength {
                expected: Self::BYTE_LEN,
                got: buf.len(),
            })?,
        ))))
    }
}

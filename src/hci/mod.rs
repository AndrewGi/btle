//! HCI Layer (where most the magic happens). Implements a Bluetooth Adapter for any controller
//! supporting HCI streams.
//! (HCI Layer is Little Endian).
pub mod adapters;
pub mod command;
pub mod event;
pub mod le;
pub mod link_control;
pub mod packet;
#[cfg(all(feature = "remote"))]
pub mod remote;
#[cfg(all(unix, feature = "bluez"))]
pub mod socket;
pub mod stream;
use crate::bytes::ToFromBytesEndian;
use crate::{ConversionError, PackError};
use core::convert::{TryFrom, TryInto};
use core::fmt::Formatter;

pub const MAX_ACL_SIZE: usize = (1492 + 4);
pub const MAX_SCO_SIZE: usize = 255;
pub const MAX_EVENT_SIZE: usize = 260;
pub const MAX_FRAME_SIZE: usize = MAX_ACL_SIZE + 4;

pub enum DeviceEvent {
    Reg = 1,
    Unreg = 2,
    Up = 3,
    Down = 4,
    Suspend = 5,
    Resume = 6,
}
pub enum BusType {
    Virtual = 0,
    USB = 1,
    PCCard = 2,
    UART = 3,
    RS232 = 4,
    PCI = 5,
    SDIO = 6,
}
pub enum ControllerType {
    BREDR = 0x00,
    AMP = 0x01,
}
/// Bluetooth Version reported by HCI Controller according to HCISpec. More versions may be added in
/// the future once they are released.
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum Version {
    Bluetooth1v0b = 0,
    Bluetooth1v1 = 1,
    Bluetooth1v2 = 2,
    Bluetooth2v0 = 3,
    Bluetooth2v1 = 4,
    Bluetooth3v0 = 5,
    Bluetooth4v0 = 6,
    Bluetooth4v1 = 7,
    Bluetooth4v2 = 8,
    Bluetooth5v0 = 9,
    Bluetooth5v1 = 10,
    Bluetooth5v2 = 11,
}
impl From<Version> for u8 {
    fn from(v: Version) -> Self {
        v as u8
    }
}
impl TryFrom<u8> for Version {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Version::Bluetooth1v0b),
            1 => Ok(Version::Bluetooth1v1),
            2 => Ok(Version::Bluetooth1v2),
            3 => Ok(Version::Bluetooth2v0),
            4 => Ok(Version::Bluetooth2v1),
            5 => Ok(Version::Bluetooth3v0),
            6 => Ok(Version::Bluetooth4v0),
            7 => Ok(Version::Bluetooth4v1),
            8 => Ok(Version::Bluetooth4v2),
            9 => Ok(Version::Bluetooth5v0),
            10 => Ok(Version::Bluetooth5v1),
            11 => Ok(Version::Bluetooth5v2),
            _ => Err(ConversionError(())),
        }
    }
}
/// HCI Error Code. Usually returned from an HCI Controller after each sent command.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum ErrorCode {
    Ok = 0x00,
    UnknownHCICommand = 0x01,
    NoConnection = 0x02,
    HardwareFailure = 0x03,
    PageTimeout = 0x04,
    AuthenticationFailure = 0x05,
    KeyMissing = 0x06,
    MemoryFull = 0x07,
    ConnectionTimeout = 0x08,
    MaxNumberOfConnections = 0x09,
    MaxNumberOfSCOConnectionsToADevice = 0x0A,
    ACLConnectionAlreadyExists = 0x0B,
    CommandDisallowed = 0x0C,
    HostRejectedDueToLimitedResources = 0x0D,
    HostRejectedDueToSecurityReasons = 0x0E,
    HostRejectedDueToARemoteDeviceOnlyAPersonalDevice = 0x0F,
    HostTimeout = 0x10,
    UnsupportedFeatureOrParameterValue = 0x11,
    InvalidHCICommandParameters = 0x12,
    OtherEndTerminatedConnectionUserEndedConnection = 0x13,
    OtherEndTerminatedConnectionLowResources = 0x14,
    OtherEndTerminatedConnectionAboutToPowerOff = 0x15,
    ConnectionTerminatedByLocalHost = 0x16,
    RepeatedAttempts = 0x17,
    PairingNotAllowed = 0x18,
    UnknownLMPPDU = 0x19,
    UnsupportedRemoteFeature = 0x1A,
    SCOOffsetRejected = 0x1B,
    SCOIntervalRejected = 0x1C,
    SCOAirModeRejected = 0x1D,
    InvalidLMPParameters = 0x1E,
    UnspecifiedError = 0x1F,
    UnsupportedLMPParameter = 0x20,
    RoleChangeNotAllowed = 0x21,
    LMPResponseTimeout = 0x22,
    LMPErrorTransactionCollision = 0x23,
    LMPPDUNotAllowed = 0x24,
    EncryptionModeNotAcceptable = 0x25,
    UnitKeyUsed = 0x26,
    QoSNotSupported = 0x27,
    InstantPassed = 0x28,
    PairingWithUnitKeyNotSupported = 0x29,
    TransactionCollision = 0x2A,
    QOSUnacceptableParameter = 0x2C,
    QOSRejected = 0x2D,
    ClassificationNotSupported = 0x2E,
    InsufficientSecurity = 0x2F,
    ParameterOutOfRange = 0x30,
    RoleSwitchPending = 0x32,
    SlotViolation = 0x34,
    RoleSwitchFailed = 0x35,
    EIRTooLarge = 0x36,
    SimplePairingNotSupported = 0x37,
    HostBusyPairing = 0x38,
}
impl ErrorCode {
    pub fn is_ok(self) -> bool {
        match self {
            ErrorCode::Ok => true,
            _ => false,
        }
    }
    pub fn error(self) -> Result<(), ErrorCode> {
        match self {
            ErrorCode::Ok => Ok(()),
            e => Err(e),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::Ok => "Ok",
            ErrorCode::UnknownHCICommand => "UnknownHCICommand",
            ErrorCode::NoConnection => "NoConnection",
            ErrorCode::HardwareFailure => "HardwareFailure",
            ErrorCode::PageTimeout => "PageTimeout",
            ErrorCode::AuthenticationFailure => "AuthenticationFailure",
            ErrorCode::KeyMissing => "KeyMissing",
            ErrorCode::MemoryFull => "MemoryFull",
            ErrorCode::ConnectionTimeout => "ConnectionTimeout",
            ErrorCode::MaxNumberOfConnections => "MaxNumberOfConnections",
            ErrorCode::MaxNumberOfSCOConnectionsToADevice => "MaxNumberOfSCOConnectionsToADevice",
            ErrorCode::ACLConnectionAlreadyExists => "ACLConnectionAlreadyExists",
            ErrorCode::CommandDisallowed => "CommandDisallowed",
            ErrorCode::HostRejectedDueToLimitedResources => "HostRejectedDueToLimitedResources",
            ErrorCode::HostRejectedDueToSecurityReasons => "HostRejectedDueToSecurityReasons",
            ErrorCode::HostRejectedDueToARemoteDeviceOnlyAPersonalDevice => {
                "HostRejectedDueToARemoteDeviceOnlyAPersonalDevice"
            }
            ErrorCode::HostTimeout => "HostTimeout",
            ErrorCode::UnsupportedFeatureOrParameterValue => "UnsupportedFeatureOrParameterValue",
            ErrorCode::InvalidHCICommandParameters => "InvalidHCICommandParameters",
            ErrorCode::OtherEndTerminatedConnectionUserEndedConnection => {
                "OtherEndTerminatedConnectionUserEndedConnection"
            }
            ErrorCode::OtherEndTerminatedConnectionLowResources => {
                "OtherEndTerminatedConnectionLowResources"
            }
            ErrorCode::OtherEndTerminatedConnectionAboutToPowerOff => {
                "OtherEndTerminatedConnectionAboutToPowerOff"
            }
            ErrorCode::ConnectionTerminatedByLocalHost => "ConnectionTerminatedByLocalHost",
            ErrorCode::RepeatedAttempts => "RepeatedAttempts",
            ErrorCode::PairingNotAllowed => "PairingNotAllowed",
            ErrorCode::UnknownLMPPDU => "UnknownLMPPDU",
            ErrorCode::UnsupportedRemoteFeature => "UnsupportedRemoteFeature",
            ErrorCode::SCOOffsetRejected => "SCOOffsetRejected",
            ErrorCode::SCOIntervalRejected => "SCOIntervalRejected",
            ErrorCode::SCOAirModeRejected => "SCOAirModeRejected",
            ErrorCode::InvalidLMPParameters => "InvalidLMPParameters",
            ErrorCode::UnspecifiedError => "UnspecifiedError",
            ErrorCode::UnsupportedLMPParameter => "UnsupportedLMPParameter",
            ErrorCode::RoleChangeNotAllowed => "RoleChangeNotAllowed",
            ErrorCode::LMPResponseTimeout => "LMPResponseTimeout",
            ErrorCode::LMPErrorTransactionCollision => "LMPErrorTransactionCollision",
            ErrorCode::LMPPDUNotAllowed => "LMPPDUNotAllowed",
            ErrorCode::EncryptionModeNotAcceptable => "EncryptionModeNotAcceptable",
            ErrorCode::UnitKeyUsed => "UnitKeyUsed",
            ErrorCode::QoSNotSupported => "QoSNotSupported",
            ErrorCode::InstantPassed => "InstantPassed",
            ErrorCode::PairingWithUnitKeyNotSupported => "PairingWithUnitKeyNotSupported",
            ErrorCode::TransactionCollision => "TransactionCollision",
            ErrorCode::QOSUnacceptableParameter => "QOSUnacceptableParameter",
            ErrorCode::QOSRejected => "QOSRejected",
            ErrorCode::ClassificationNotSupported => "ClassificationNotSupported",
            ErrorCode::InsufficientSecurity => "InsufficientSecurity",
            ErrorCode::ParameterOutOfRange => "ParameterOutOfRange",
            ErrorCode::RoleSwitchPending => "RoleSwitchPending",
            ErrorCode::SlotViolation => "SlotViolation",
            ErrorCode::RoleSwitchFailed => "RoleSwitchFailed",
            ErrorCode::EIRTooLarge => "EIRTooLarge",
            ErrorCode::SimplePairingNotSupported => "SimplePairingNotSupported",
            ErrorCode::HostBusyPairing => "HostBusyPairing",
        }
    }
}
impl core::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<ErrorCode> for u8 {
    fn from(code: ErrorCode) -> Self {
        code as u8
    }
}
impl TryFrom<u8> for ErrorCode {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ErrorCode::Ok),
            0x01 => Ok(ErrorCode::UnknownHCICommand),
            0x02 => Ok(ErrorCode::NoConnection),
            0x03 => Ok(ErrorCode::HardwareFailure),
            0x04 => Ok(ErrorCode::PageTimeout),
            0x05 => Ok(ErrorCode::AuthenticationFailure),
            0x06 => Ok(ErrorCode::KeyMissing),
            0x07 => Ok(ErrorCode::MemoryFull),
            0x08 => Ok(ErrorCode::ConnectionTimeout),
            0x09 => Ok(ErrorCode::MaxNumberOfConnections),
            0x0A => Ok(ErrorCode::MaxNumberOfSCOConnectionsToADevice),
            0x0B => Ok(ErrorCode::ACLConnectionAlreadyExists),
            0x0C => Ok(ErrorCode::CommandDisallowed),
            0x0D => Ok(ErrorCode::HostRejectedDueToLimitedResources),
            0x0E => Ok(ErrorCode::HostRejectedDueToSecurityReasons),
            0x0F => Ok(ErrorCode::HostRejectedDueToARemoteDeviceOnlyAPersonalDevice),
            0x10 => Ok(ErrorCode::HostTimeout),
            0x11 => Ok(ErrorCode::UnsupportedFeatureOrParameterValue),
            0x12 => Ok(ErrorCode::InvalidHCICommandParameters),
            0x13 => Ok(ErrorCode::OtherEndTerminatedConnectionUserEndedConnection),
            0x14 => Ok(ErrorCode::OtherEndTerminatedConnectionLowResources),
            0x15 => Ok(ErrorCode::OtherEndTerminatedConnectionAboutToPowerOff),
            0x16 => Ok(ErrorCode::ConnectionTerminatedByLocalHost),
            0x17 => Ok(ErrorCode::RepeatedAttempts),
            0x18 => Ok(ErrorCode::PairingNotAllowed),
            0x19 => Ok(ErrorCode::UnknownLMPPDU),
            0x1A => Ok(ErrorCode::UnsupportedRemoteFeature),
            0x1B => Ok(ErrorCode::SCOOffsetRejected),
            0x1C => Ok(ErrorCode::SCOIntervalRejected),
            0x1D => Ok(ErrorCode::SCOAirModeRejected),
            0x1E => Ok(ErrorCode::InvalidLMPParameters),
            0x1F => Ok(ErrorCode::UnspecifiedError),
            0x20 => Ok(ErrorCode::UnsupportedLMPParameter),
            0x21 => Ok(ErrorCode::RoleChangeNotAllowed),
            0x22 => Ok(ErrorCode::LMPResponseTimeout),
            0x23 => Ok(ErrorCode::LMPErrorTransactionCollision),
            0x24 => Ok(ErrorCode::LMPPDUNotAllowed),
            0x25 => Ok(ErrorCode::EncryptionModeNotAcceptable),
            0x26 => Ok(ErrorCode::UnitKeyUsed),
            0x27 => Ok(ErrorCode::QoSNotSupported),
            0x28 => Ok(ErrorCode::InstantPassed),
            0x29 => Ok(ErrorCode::PairingWithUnitKeyNotSupported),
            _ => Err(ConversionError(())),
        }
    }
}
pub const EVENT_MAX_LEN: usize = 255;
pub const COMMAND_MAX_LEN: usize = 255;
pub const FULL_COMMAND_MAX_LEN: usize = COMMAND_MAX_LEN + OPCODE_LEN + 1;
pub const EVENT_CODE_LEN: usize = 1;
/// 6 bit OGF. (OpCode Ground Field)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum OGF {
    NOP = 0x00,
    LinkControl = 0x01,
    LinkPolicy = 0x02,
    HCIControlBaseband = 0x03,
    InformationalParameters = 0x04,
    StatusParameters = 0x05,
    Testing = 0x06,
    LEController = 0x08,
    VendorSpecific = 0x3F,
}
impl Default for OGF {
    fn default() -> Self {
        OGF::NOP
    }
}
impl From<OGF> for u8 {
    fn from(ogf: OGF) -> Self {
        ogf as u8
    }
}
impl TryFrom<u8> for OGF {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OGF::NOP),
            0x01 => Ok(OGF::LinkControl),
            0x02 => Ok(OGF::LinkPolicy),
            0x03 => Ok(OGF::HCIControlBaseband),
            0x04 => Ok(OGF::InformationalParameters),
            0x05 => Ok(OGF::StatusParameters),
            0x06 => Ok(OGF::Testing),
            0x08 => Ok(OGF::LEController),
            0x3F => Ok(OGF::VendorSpecific),
            _ => Err(ConversionError(())),
        }
    }
}
pub const OCF_MAX: u16 = (1 << 10) - 1;
/// 10 bit OCF (`Opcode` Command Field)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct OCF(u16);
impl OCF {
    /// Creates a new 10-bit OCF
    /// # Panics
    /// Panics if `ocf > OCF_MAX` (if `ocf` isn't 10-bit)
    pub fn new(ocf: u16) -> Self {
        assert!(ocf <= OCF_MAX, "ocf bigger than 10 bits");
        Self(ocf)
    }
    /// Creates a new 10-bit OCF by masking a u16
    pub fn new_masked(ocf: u16) -> Self {
        Self(ocf & OCF_MAX)
    }
}
impl From<OCF> for u16 {
    fn from(ocf: OCF) -> Self {
        ocf.0
    }
}
pub const OPCODE_LEN: usize = 2;
/// 16-bit HCI Opcode. Contains a OGF (OpCode Ground Field) and OCF (OpCode Command Field).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Opcode(pub OGF, pub OCF);
impl Opcode {
    pub const fn byte_len() -> usize {
        OPCODE_LEN
    }
    /// # Errors
    /// returns `HCIPackError::BadLength` if `buf.len() != OPCODE_LEN`.
    pub fn pack(self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(OPCODE_LEN, buf)?;
        buf[..2].copy_from_slice(&u16::from(self).to_bytes_le());
        Ok(())
    }
    /// # Errors
    /// returns `HCIPackError::BadLength` if `buf.len() != OPCODE_LEN`.
    /// returns `HCIPackError::BadBytes` if `buf` doesn't contain a value opcode.
    pub fn unpack(buf: &[u8]) -> Result<Opcode, PackError> {
        PackError::expect_length(OPCODE_LEN, buf)?;
        Ok(u16::from_bytes_le(&buf)
            .expect("length checked above")
            .try_into()
            .ok()
            .ok_or(PackError::BadBytes { index: Some(0) })?)
    }
    pub const fn nop() -> Opcode {
        Opcode(OGF::NOP, OCF(0))
    }
    pub fn is_nop(self) -> bool {
        self.0 == OGF::NOP
    }
}
impl From<Opcode> for u16 {
    fn from(opcode: Opcode) -> Self {
        (opcode.1).0 | (u16::from(u8::from(opcode.0)) << 10)
    }
}
impl TryFrom<u16> for Opcode {
    type Error = ConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let ogf = OGF::try_from(u8::try_from(value >> 10).expect("OGF is 6-bits"))?;
        let ocf = OCF::new_masked(value);
        Ok(Opcode(ogf, ocf))
    }
}

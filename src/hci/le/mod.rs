//! HCI LE Layer. Handles everything from advertising, scanning, LE links, etc.
pub mod advertise;
pub mod mask;
pub mod messages;
pub mod report;
pub use messages::*;
pub mod random;
pub mod scan;
use crate::hci::event::{Event, EventCode};
use crate::hci::{Opcode, OCF, OGF};
use crate::{ConversionError, PackError};
use core::convert::TryFrom;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(u16)]
pub enum LEControllerOpcode {
    SetEventMask = 0x0001,
    ReadBufferSize = 0x0002,
    ReadLocalSupportedFeatures = 0x0003,
    SetRandomAddress = 0x0005,
    SetAdvertisingParameters = 0x0006,
    ReadAdvertisingChannelTxPower = 0x0007,
    SetAdvertisingData = 0x0008,
    SetScanResponseData = 0x0009,
    SetAdvertisingEnable = 0x000A,
    SetScanParameters = 0x000B,
    SetScanEnable = 0x000C,
    CreateConnection = 0x000D,
    CreateConnectionCancel = 0x000E,
    ReadWhitelistSize = 0x000F,
    ClearWhitelist = 0x0010,
    AddDeviceToWhitelist = 0x0011,
    RemoveDeviceFromWhitelist = 0x0012,
    ConnectionUpdate = 0x0013,
    SetHostChannelClassification = 0x0014,
    ReadChannelMap = 0x0015,
    ReadRemoteUsedFeatures = 0x0016,
    Encrypt = 0x0017,
    Rand = 0x0018,
    StartEncryption = 0x0019,
    LongTermKeyRequestReply = 0x001A,
    LongTermKeyRequestNegativeReply = 0x001B,
    ReadSupportedState = 0x001C,
    ReceiverTest = 0x001D,
    TransmitterTest = 0x001E,
    TestEnd = 0x001F,
}
impl TryFrom<OCF> for LEControllerOpcode {
    type Error = ConversionError;

    fn try_from(ocf: OCF) -> Result<Self, Self::Error> {
        match u16::from(ocf) {
            0x0001 => Ok(LEControllerOpcode::SetEventMask),
            0x0002 => Ok(LEControllerOpcode::ReadBufferSize),
            0x0003 => Ok(LEControllerOpcode::ReadLocalSupportedFeatures),
            0x0005 => Ok(LEControllerOpcode::SetRandomAddress),
            0x0006 => Ok(LEControllerOpcode::SetAdvertisingParameters),
            0x0007 => Ok(LEControllerOpcode::ReadAdvertisingChannelTxPower),
            0x0008 => Ok(LEControllerOpcode::SetAdvertisingData),
            0x0009 => Ok(LEControllerOpcode::SetScanResponseData),
            0x000A => Ok(LEControllerOpcode::SetAdvertisingEnable),
            0x000B => Ok(LEControllerOpcode::SetScanParameters),
            0x000C => Ok(LEControllerOpcode::SetScanEnable),
            0x000D => Ok(LEControllerOpcode::CreateConnection),
            0x000E => Ok(LEControllerOpcode::CreateConnectionCancel),
            0x000F => Ok(LEControllerOpcode::ReadWhitelistSize),
            0x0010 => Ok(LEControllerOpcode::ClearWhitelist),
            0x0011 => Ok(LEControllerOpcode::AddDeviceToWhitelist),
            0x0012 => Ok(LEControllerOpcode::RemoveDeviceFromWhitelist),
            0x0013 => Ok(LEControllerOpcode::ConnectionUpdate),
            0x0014 => Ok(LEControllerOpcode::SetHostChannelClassification),
            0x0015 => Ok(LEControllerOpcode::ReadChannelMap),
            0x0016 => Ok(LEControllerOpcode::ReadRemoteUsedFeatures),
            0x0017 => Ok(LEControllerOpcode::Encrypt),
            0x0018 => Ok(LEControllerOpcode::Rand),
            0x0019 => Ok(LEControllerOpcode::StartEncryption),
            0x001A => Ok(LEControllerOpcode::LongTermKeyRequestReply),
            0x001B => Ok(LEControllerOpcode::LongTermKeyRequestNegativeReply),
            0x001C => Ok(LEControllerOpcode::ReadSupportedState),
            0x001D => Ok(LEControllerOpcode::ReceiverTest),
            0x001E => Ok(LEControllerOpcode::TransmitterTest),
            0x001F => Ok(LEControllerOpcode::TestEnd),
            _ => Err(ConversionError(())),
        }
    }
}
impl LEControllerOpcode {
    pub const fn ogf() -> OGF {
        OGF::LEController
    }
}
impl From<LEControllerOpcode> for OCF {
    fn from(opcode: LEControllerOpcode) -> Self {
        OCF::new(opcode as u16)
    }
}
impl From<LEControllerOpcode> for Opcode {
    fn from(opcode: LEControllerOpcode) -> Self {
        Opcode(OGF::LEController, opcode.into())
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum MetaEventCode {
    ConnectionComplete = 0x01,
    AdvertisingReport = 0x02,
    ConnectionUpdateComplete = 0x03,
    ReadRemoteFeatures = 0x04,
    LongTermKeyRequest = 0x05,
    RemoteConnectionParametersRequest = 0x06,
    DataLengthChange = 0x07,
    ReadLocalP256PublicKeyComplete = 0x08,
    GenerateDHKeyComplete = 0x09,
    EnhancedConnectionComplete = 0x0A,
    DirectedAdvertisingReport = 0x0B,
    PHYUpdateCompleteEvent = 0x0C,
    ExtendedAdvertisingReport = 0x0D,
    PeriodicAdvertisingSyncEstablished = 0x0E,
    PeriodicAdvertisingReport = 0x0F,
    PeriodicAdvertisingSyncLost = 0x10,
    ScanTimeout = 0x11,
    AdvertisingSetTerminated = 0x12,
    ScanRequestReceived = 0x13,
    ChannelSelectionAlgorithm = 0x14,
    ConnectionlessIQReport = 0x15,
    ConnectionIQReport = 0x16,
    CTERequestFailed = 0x17,
    PeriodicAdvertisingSyncTransferReceived = 0x18,
    CISEstablished = 0x19,
    CISRequest = 0x1A,
    CreateBIGComplete = 0x1B,
    TerminateBIGComplete = 0x1C,
    BIGSyncEstablished = 0x1D,
    BIGSyncLost = 0x1E,
    RequestPeerSCAComplete = 0x1F,
    PathLossThreshold = 0x20,
    TransmitPowerReporting = 0x21,
    BIGInfoAdvertisingReport = 0x22,
}
impl MetaEventCode {
    /// The `MetaEventCode` with the highest value.
    pub const MAX_CODE: MetaEventCode = MetaEventCode::BIGInfoAdvertisingReport;
}
impl From<MetaEventCode> for u8 {
    fn from(c: MetaEventCode) -> Self {
        c as u8
    }
}
impl TryFrom<u8> for MetaEventCode {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(MetaEventCode::ConnectionComplete),
            0x02 => Ok(MetaEventCode::AdvertisingReport),
            0x03 => Ok(MetaEventCode::ConnectionUpdateComplete),
            0x04 => Ok(MetaEventCode::ReadRemoteFeatures),
            0x05 => Ok(MetaEventCode::LongTermKeyRequest),
            0x06 => Ok(MetaEventCode::RemoteConnectionParametersRequest),
            0x07 => Ok(MetaEventCode::DataLengthChange),
            0x08 => Ok(MetaEventCode::ReadLocalP256PublicKeyComplete),
            0x09 => Ok(MetaEventCode::GenerateDHKeyComplete),
            0x0A => Ok(MetaEventCode::EnhancedConnectionComplete),
            0x0B => Ok(MetaEventCode::DirectedAdvertisingReport),
            0x0C => Ok(MetaEventCode::PHYUpdateCompleteEvent),
            0x0D => Ok(MetaEventCode::ExtendedAdvertisingReport),
            0x0E => Ok(MetaEventCode::PeriodicAdvertisingSyncEstablished),
            0x0F => Ok(MetaEventCode::PeriodicAdvertisingReport),
            0x10 => Ok(MetaEventCode::PeriodicAdvertisingSyncLost),
            0x11 => Ok(MetaEventCode::ScanTimeout),
            0x12 => Ok(MetaEventCode::AdvertisingSetTerminated),
            0x13 => Ok(MetaEventCode::ScanRequestReceived),
            0x14 => Ok(MetaEventCode::ChannelSelectionAlgorithm),
            0x15 => Ok(MetaEventCode::ConnectionlessIQReport),
            0x16 => Ok(MetaEventCode::ConnectionIQReport),
            0x17 => Ok(MetaEventCode::CTERequestFailed),
            0x18 => Ok(MetaEventCode::PeriodicAdvertisingSyncTransferReceived),
            0x19 => Ok(MetaEventCode::CISEstablished),
            0x1A => Ok(MetaEventCode::CISRequest),
            0x1B => Ok(MetaEventCode::CreateBIGComplete),
            0x1C => Ok(MetaEventCode::TerminateBIGComplete),
            0x1D => Ok(MetaEventCode::BIGSyncEstablished),
            0x1E => Ok(MetaEventCode::BIGSyncLost),
            0x1F => Ok(MetaEventCode::RequestPeerSCAComplete),
            0x20 => Ok(MetaEventCode::PathLossThreshold),
            0x21 => Ok(MetaEventCode::TransmitPowerReporting),
            0x22 => Ok(MetaEventCode::BIGInfoAdvertisingReport),
            _ => Err(ConversionError(())),
        }
    }
}
pub trait MetaEvent {
    const META_CODE: MetaEventCode;
    fn byte_len(&self) -> usize;
    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError>;
}
impl<M: MetaEvent> Event for M {
    const CODE: EventCode = EventCode::LEMeta;

    fn byte_len(&self) -> usize {
        MetaEvent::byte_len(self) + 1
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        if u8::from(Self::META_CODE)
            == *buf.get(0).ok_or(PackError::BadLength {
                expected: 1,
                got: 0,
            })?
        {
            MetaEvent::unpack_from(&buf[1..])
        } else {
            Err(PackError::bad_index(0))
        }
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.byte_len() + 1, buf)?;
        <Self as MetaEvent>::pack_into(self, &mut buf[1..])?;
        buf[0] = Self::META_CODE.into();
        Ok(())
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum OwnAddressType {
    Public = 0x00,
    Random = 0x01,
    PrivateOrPublic = 0x02,
    PrivateOrRandom = 0x03,
}

impl From<OwnAddressType> for u8 {
    fn from(s: OwnAddressType) -> Self {
        s as u8
    }
}
impl TryFrom<u8> for OwnAddressType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OwnAddressType::Public),
            1 => Ok(OwnAddressType::Random),
            2 => Ok(OwnAddressType::PrivateOrPublic),
            3 => Ok(OwnAddressType::PrivateOrRandom),
            _ => Err(ConversionError(())),
        }
    }
}

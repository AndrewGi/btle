use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::StatusReturn;
use crate::hci::{HCIConversionError, HCIPackError, Opcode, OCF, OGF};
use core::convert::{TryFrom, TryInto};

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
    type Error = HCIConversionError;

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
            _ => Err(HCIConversionError(())),
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
pub enum LEMetaEventCode {
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
    PeriodicAdvertisingSyncEstablished = 0xE,
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

pub trait LEMetaEvent {
    const CODE: LEMetaEventCode;
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetScanEnable {
    pub is_enabled: bool,
    pub filter_duplicates: bool,
}
impl SetScanEnable {
    pub const BYTE_LEN: usize = 2;
}
impl Command for SetScanEnable {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetScanEnable.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        HCIPackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.is_enabled.into();
        buf[1] = self.filter_duplicates.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        HCIPackError::expect_length(Self::BYTE_LEN, buf)?;
        let is_enabled = match buf[0] {
            0 => false,
            1 => true,
            _ => return Err(HCIPackError::bad_index(0)),
        };
        let filter_duplicates = match buf[1] {
            0 => false,
            1 => true,
            _ => return Err(HCIPackError::bad_index(1)),
        };
        Ok(Self {
            is_enabled,
            filter_duplicates,
        })
    }
}
impl SetScanEnable {}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetAdvertisingEnable {
    pub is_enabled: bool,
}
const SET_ADVERTISING_ENABLE_LEN: usize = 1;
impl Command for SetAdvertisingEnable {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetAdvertisingEnable.into()
    }

    fn byte_len(&self) -> usize {
        SET_ADVERTISING_ENABLE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        HCIPackError::expect_length(SET_ADVERTISING_ENABLE_LEN, buf)?;
        buf[0] = self.is_enabled.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        HCIPackError::expect_length(SET_ADVERTISING_ENABLE_LEN, buf)?;
        match buf[0] {
            0 => Ok(Self { is_enabled: false }),
            1 => Ok(Self { is_enabled: true }),
            _ => Err(HCIPackError::bad_index(0)),
        }
    }
}
const ADVERTISING_DATA_MAX_LEN: usize = 0x1F;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetAdvertisingData {
    data: [u8; ADVERTISING_DATA_MAX_LEN],
    len: u8,
}

impl Command for SetAdvertisingData {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetAdvertisingData.into()
    }

    fn byte_len(&self) -> usize {
        usize::from(self.len) + 1
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        HCIPackError::expect_length(self.byte_len(), buf)?;
        buf[0] = self.len;
        let l = usize::from(self.len);
        buf[1..][..l].copy_from_slice(&self.data[..l]);
        Ok(())
    }

    fn unpack_from(_buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
impl SetAdvertisingData {
    pub fn new(data: &[u8]) -> SetAdvertisingData {
        assert!(data.len() <= ADVERTISING_DATA_MAX_LEN);
        let mut buf = [0_u8; ADVERTISING_DATA_MAX_LEN];
        buf[..data.len()].copy_from_slice(data);
        SetAdvertisingData {
            data: buf,
            len: data.len().try_into().expect("data max len 0x1F"),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum ScanType {
    Passive = 0x00,
    Active = 0x01,
}
impl From<ScanType> for u8 {
    fn from(s: ScanType) -> Self {
        s as u8
    }
}
impl TryFrom<u8> for ScanType {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanType::Passive),
            1 => Ok(ScanType::Active),
            _ => Err(HCIConversionError(())),
        }
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
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OwnAddressType::Public),
            1 => Ok(OwnAddressType::Random),
            2 => Ok(OwnAddressType::PrivateOrPublic),
            3 => Ok(OwnAddressType::PrivateOrRandom),
            _ => Err(HCIConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum ScanningFilterPolicy {
    All = 0x00,
    Whitelisted = 0x01,
    DirectedAll = 0x02,
    DirectedWhitelisted = 0x03,
}
impl From<ScanningFilterPolicy> for u8 {
    fn from(p: ScanningFilterPolicy) -> Self {
        p as u8
    }
}
impl TryFrom<u8> for ScanningFilterPolicy {
    type Error = HCIConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScanningFilterPolicy::All),
            1 => Ok(ScanningFilterPolicy::Whitelisted),
            2 => Ok(ScanningFilterPolicy::DirectedAll),
            3 => Ok(ScanningFilterPolicy::DirectedWhitelisted),
            _ => Err(HCIConversionError(())),
        }
    }
}
/// Range 0x0004 --> 0x4000
/// Default 0x0010 (10 ms)
/// Time = N *  0.625 ms
/// Time Range 2.5 ms --> 10.24 s
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ScanInterval(pub u16);
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ScanWindow(pub u16);
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SetScanParameters {
    pub scan_type: ScanType,
    pub scan_internal: ScanInterval,
    pub scan_window: ScanWindow,
    pub own_address_type: OwnAddressType,
    pub scanning_filter_policy: ScanningFilterPolicy,
}
pub const SET_SCAN_PARAMETERS_LEN: usize = 7;
impl Command for SetScanParameters {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetScanParameters.into()
    }

    fn byte_len(&self) -> usize {
        SET_SCAN_PARAMETERS_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), HCIPackError> {
        HCIPackError::expect_length(SET_SCAN_PARAMETERS_LEN, buf)?;
        buf[0] = self.scan_type.into();
        buf[1..3].copy_from_slice(&self.scan_internal.0.to_bytes_le()[..]);
        buf[3..5].copy_from_slice(&self.scan_window.0.to_bytes_le()[..]);
        buf[5] = self.own_address_type.into();
        buf[6] = self.scanning_filter_policy.into();
        Ok(())
    }

    fn unpack_from(_buf: &[u8]) -> Result<Self, HCIPackError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

//! LE [`SetAdvertisingEnable`], [`SetAdvertisingData`] and other advertising types.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::{ReturnParameters, StatusReturn};
use crate::hci::le::LEControllerOpcode;
use crate::hci::{ErrorCode, Opcode};
use crate::{BTAddress, ConversionError, PackError, BT_ADDRESS_LEN, RSSI};
use core::convert::{TryFrom, TryInto};

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

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(SET_ADVERTISING_ENABLE_LEN, buf)?;
        buf[0] = self.is_enabled.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(SET_ADVERTISING_ENABLE_LEN, buf)?;
        match buf[0] {
            0 => Ok(Self { is_enabled: false }),
            1 => Ok(Self { is_enabled: true }),
            _ => Err(PackError::bad_index(0)),
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

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(self.byte_len(), buf)?;
        buf[0] = self.len;
        let l = usize::from(self.len);
        buf[1..][..l].copy_from_slice(&self.data[..l]);
        Ok(())
    }

    fn unpack_from(_buf: &[u8]) -> Result<Self, PackError>
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
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct AdvertisingInterval(u16);
impl AdvertisingInterval {
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x0020u16;
    pub const MIN: AdvertisingInterval = AdvertisingInterval(Self::MIN_U16);
    pub const MAX_U16: u16 = 0x4000u16;
    pub const MAX: AdvertisingInterval = AdvertisingInterval(Self::MAX_U16);
    pub const DEFAULT_U16: u16 = 0x0800u16;
    pub const DEFAULT: AdvertisingInterval = AdvertisingInterval(Self::DEFAULT_U16);
    /// Creates a new `AdvertisingInterval`.
    /// # Panics
    /// Panics if
    /// `interval < AdvertisingInterval::MIN_U16 || interval > AdvertisingInterval::MAX_U16`.
    pub fn new(interval: u16) -> AdvertisingInterval {
        assert!(
            interval <= Self::MAX_U16 && interval >= Self::MIN_U16,
            "invalid advertising interval '{}'",
            interval
        );
        AdvertisingInterval(interval)
    }
    pub fn as_microseconds(self) -> u32 {
        u32::from(u16::from(self)) * 625
    }
}
impl Default for AdvertisingInterval {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl TryFrom<u16> for AdvertisingInterval {
    type Error = ConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::MAX_U16 && value >= Self::MIN_U16 {
            Ok(Self(value))
        } else {
            Err(ConversionError(()))
        }
    }
}
impl From<AdvertisingInterval> for u16 {
    fn from(a: AdvertisingInterval) -> Self {
        a.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AdvertisingType {
    AdvInd = 0x00,
    AdvDirectIndHighDutyCycle = 0x01,
    AdvScanInd = 0x02,
    AdvNonnconnInd = 0x03,
    AdvDirectIndLowDutyCycle = 0x04,
}
impl AdvertisingType {
    pub const DEFAULT: AdvertisingType = AdvertisingType::AdvInd;
}
impl Default for AdvertisingType {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl From<AdvertisingType> for u8 {
    fn from(a: AdvertisingType) -> Self {
        a as u8
    }
}
impl TryFrom<u8> for AdvertisingType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(AdvertisingType::AdvInd),
            0x01 => Ok(AdvertisingType::AdvDirectIndHighDutyCycle),
            0x02 => Ok(AdvertisingType::AdvScanInd),
            0x03 => Ok(AdvertisingType::AdvNonnconnInd),
            0x04 => Ok(AdvertisingType::AdvDirectIndLowDutyCycle),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PeerAddressType {
    Public = 0x00,
    Random = 0x01,
}
impl PeerAddressType {
    pub const DEFAULT: PeerAddressType = PeerAddressType::Public;
}
impl Default for PeerAddressType {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl From<PeerAddressType> for u8 {
    fn from(a: PeerAddressType) -> Self {
        a as u8
    }
}
impl TryFrom<u8> for PeerAddressType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PeerAddressType::Public),
            0x01 => Ok(PeerAddressType::Random),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum OwnAddressType {
    PublicDevice = 0x00,
    RandomDevice = 0x01,
    PrivateOrPublic = 0x02,
    PrivateOrRandom = 0x03,
}
impl OwnAddressType {
    pub const DEFAULT: OwnAddressType = OwnAddressType::PublicDevice;
}
impl Default for OwnAddressType {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl From<OwnAddressType> for u8 {
    fn from(t: OwnAddressType) -> Self {
        t as u8
    }
}
impl TryFrom<u8> for OwnAddressType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OwnAddressType::PublicDevice),
            0x01 => Ok(OwnAddressType::RandomDevice),
            0x02 => Ok(OwnAddressType::PrivateOrPublic),
            0x03 => Ok(OwnAddressType::PrivateOrRandom),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Channels {
    Channel37 = 0x00,
    Channel38 = 0x01,
    Channel39 = 0x02,
}
impl From<Channels> for u8 {
    fn from(c: Channels) -> Self {
        c as u8
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ChannelMap(u8);
impl ChannelMap {
    pub const ZEROED: ChannelMap = ChannelMap(0);
    pub const ALL_U8: u8 = 0x07;
    pub const ALL: ChannelMap = ChannelMap(ChannelMap::ALL_U8);
    pub const DEFAULT: ChannelMap = ChannelMap::ALL;
    /// Creates a new `ChannelMap`.
    /// # Panics
    /// Panics if `map > u16::from(ChannelMap::ALL)`;
    pub fn new(map: u8) -> ChannelMap {
        assert!(map > Self::ALL_U8, "invalid channel map {}", map);
        ChannelMap(map)
    }
    pub fn enable_channel(&mut self, channel: Channels) {
        self.0 |= 1u8 << u8::from(channel);
    }
    pub fn disable_channel(&mut self, channel: Channels) {
        self.0 &= !(1u8 << u8::from(channel));
    }
    pub fn get_channel(self, channel: Channels) -> bool {
        self.0 & (1u8 << u8::from(channel)) != 0
    }
}

impl Default for ChannelMap {
    fn default() -> Self {
        ChannelMap::DEFAULT
    }
}
impl From<ChannelMap> for u8 {
    fn from(m: ChannelMap) -> Self {
        m.0
    }
}
impl TryFrom<u8> for ChannelMap {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::ALL_U8 {
            Ok(ChannelMap(value))
        } else {
            Err(ConversionError(()))
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FilterPolicy {
    /// Process scan and connection requests from all devices (i.e., the White List is not in use)
    All = 0x00,
    /// Process connection requests from all devices and scan requests only from devices that are
    /// in the White List.
    ConnectionAllScanWhitelist = 0x01,
    /// Process scan requests from all devices and connection requests only from devices that are
    /// in the White List.
    ScanAllConnectionWhitelist = 0x02,
    /// Process scan and connection requests only from devices in the White List.
    Whitelist = 0x03,
}
impl FilterPolicy {
    pub const DEFAULT: FilterPolicy = FilterPolicy::All;
}
impl Default for FilterPolicy {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl From<FilterPolicy> for u8 {
    fn from(f: FilterPolicy) -> Self {
        f as u8
    }
}
impl TryFrom<u8> for FilterPolicy {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(FilterPolicy::All),
            0x01 => Ok(FilterPolicy::ConnectionAllScanWhitelist),
            0x02 => Ok(FilterPolicy::ScanAllConnectionWhitelist),
            0x03 => Ok(FilterPolicy::Whitelist),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SetAdvertisingParameters {
    pub interval_min: AdvertisingInterval,
    pub interval_max: AdvertisingInterval,
    pub advertising_type: AdvertisingType,
    pub own_address_type: OwnAddressType,
    pub peer_address_type: PeerAddressType,
    pub peer_address: BTAddress,
    pub channel_map: ChannelMap,
    pub filter_policy: FilterPolicy,
}
impl SetAdvertisingParameters {
    /// interval_min (2) + interval_max (2) + advertising_type (1) + own_address_type (1) +
    /// peer_address_type (1) + peer_address (6) + channel_map (1) + filter_policy (1)
    pub const BYTE_LEN: usize =
        AdvertisingInterval::BYTE_LEN * 2 + 1 + 1 + 1 + BT_ADDRESS_LEN + 1 + 1;
    pub const DEFAULT: SetAdvertisingParameters = SetAdvertisingParameters {
        interval_min: AdvertisingInterval::DEFAULT,
        interval_max: AdvertisingInterval::DEFAULT,
        advertising_type: AdvertisingType::DEFAULT,
        own_address_type: OwnAddressType::DEFAULT,
        peer_address_type: PeerAddressType::DEFAULT,
        peer_address: BTAddress::ZEROED,
        channel_map: ChannelMap::DEFAULT,
        filter_policy: FilterPolicy::DEFAULT,
    };
    /// Creates a new `SetAdvertisingParameters` from `self` with `self.address` set to the
    /// `address` parameters.
    pub const fn with_address(self, address: BTAddress) -> SetAdvertisingParameters {
        SetAdvertisingParameters {
            interval_min: self.interval_max,
            interval_max: self.interval_max,
            advertising_type: self.advertising_type,
            own_address_type: self.own_address_type,
            peer_address_type: self.peer_address_type,
            peer_address: address,
            channel_map: self.channel_map,
            filter_policy: self.filter_policy,
        }
    }
}
impl Default for SetAdvertisingParameters {
    fn default() -> Self {
        Self::DEFAULT
    }
}
impl Command for SetAdvertisingParameters {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetAdvertisingParameters.into()
    }

    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0..2].copy_from_slice(&u16::from(self.interval_min).to_bytes_le()[..]);
        buf[2..4].copy_from_slice(&u16::from(self.interval_max).to_bytes_le()[..]);
        buf[4] = self.advertising_type.into();
        buf[5] = self.own_address_type.into();
        buf[6] = self.peer_address_type.into();
        self.peer_address
            .pack_into(&mut buf[7..7 + BT_ADDRESS_LEN])?;
        buf[7 + BT_ADDRESS_LEN] = self.channel_map.into();
        buf[8 + BT_ADDRESS_LEN] = self.filter_policy.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        Ok(SetAdvertisingParameters {
            interval_min: AdvertisingInterval::try_from(
                u16::from_bytes_le(&buf[0..2]).expect("hardcoded length"),
            )
            .map_err(|_| PackError::bad_index(0))?,
            interval_max: AdvertisingInterval::try_from(
                u16::from_bytes_le(&buf[2..4]).expect("hardcoded length"),
            )
            .map_err(|_| PackError::bad_index(2))?,
            advertising_type: AdvertisingType::try_from(buf[4])
                .map_err(|_| PackError::bad_index(4))?,
            own_address_type: OwnAddressType::try_from(buf[5])
                .map_err(|_| PackError::bad_index(5))?,
            peer_address_type: PeerAddressType::try_from(buf[6])
                .map_err(|_| PackError::bad_index(6))?,
            peer_address: BTAddress::unpack_from(&buf[7..7 + BT_ADDRESS_LEN])?,
            channel_map: ChannelMap::try_from(buf[7 + BT_ADDRESS_LEN])
                .map_err(|_| PackError::bad_index(7 + BT_ADDRESS_LEN))?,
            filter_policy: FilterPolicy::try_from(buf[8 + BT_ADDRESS_LEN])
                .map_err(|_| PackError::bad_index(8 + BT_ADDRESS_LEN))?,
        })
    }
}

pub struct ReadAdvertisingChannelTxPower {}
impl Command for ReadAdvertisingChannelTxPower {
    type Return = TxPowerLevelReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::ReadAdvertisingChannelTxPower.into()
    }

    fn byte_len(&self) -> usize {
        0
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(0, buf)?;
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(0, buf)?;
        Ok(ReadAdvertisingChannelTxPower {})
    }
}
/// Tx Power Level in dBm. Accuracy +-4 dB. Range (-127 to +20 dBm).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct TxPowerLevel(i8);
impl TxPowerLevel {
    pub const MIN_DBM_I8: i8 = -127;
    pub const MAX_DBM_I8: i8 = 20;
    pub const MAX: TxPowerLevel = TxPowerLevel(Self::MAX_DBM_I8);
    pub const MIN: TxPowerLevel = TxPowerLevel(Self::MIN_DBM_I8);
    /// Creates a new RSSI from `dbm`.
    /// # Panics
    /// Panics if `dbm < Self::MIN || dbm > Self::MAX`.
    pub fn new(dbm: i8) -> TxPowerLevel {
        assert!(
            dbm >= Self::MIN_DBM_I8 && dbm <= Self::MAX_DBM_I8,
            "invalid power levle '{}'",
            dbm
        );
        TxPowerLevel(dbm)
    }
}
impl From<TxPowerLevel> for i8 {
    fn from(rssi: TxPowerLevel) -> Self {
        rssi.0
    }
}

impl From<TxPowerLevel> for u8 {
    fn from(rssi: TxPowerLevel) -> Self {
        rssi.0 as u8
    }
}
impl TryFrom<i8> for TxPowerLevel {
    type Error = ConversionError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value > Self::MAX_DBM_I8 || value < Self::MIN_DBM_I8 {
            Err(ConversionError(()))
        } else {
            Ok(TxPowerLevel(value))
        }
    }
}
impl TryFrom<u8> for TxPowerLevel {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        (value as i8).try_into()
    }
}
impl From<TxPowerLevel> for RSSI {
    /// Power Level as RSSI have the same dBm range.
    fn from(l: TxPowerLevel) -> Self {
        RSSI(l.0)
    }
}
impl From<RSSI> for TxPowerLevel {
    /// Power Level as RSSI have the same dBm range.
    fn from(rssi: RSSI) -> Self {
        TxPowerLevel(rssi.0)
    }
}
pub struct TxPowerLevelReturn {
    pub status: ErrorCode,
    pub power_level: TxPowerLevel,
}
impl TxPowerLevelReturn {
    pub const BYTE_LEN: usize = 2;
}
impl ReturnParameters for TxPowerLevelReturn {
    fn byte_len(&self) -> usize {
        Self::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        buf[0] = self.status.into();
        buf[1] = self.power_level.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(Self::BYTE_LEN, buf)?;
        Ok(TxPowerLevelReturn {
            status: ErrorCode::try_from(buf[0]).map_err(|_| PackError::bad_index(0))?,
            power_level: TxPowerLevel::try_from(buf[1]).map_err(|_| PackError::bad_index(1))?,
        })
    }
}

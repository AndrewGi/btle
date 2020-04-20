//! LE [`SetAdvertisingEnable`], [`SetAdvertisingData`] and other advertising types.
use crate::bytes::ToFromBytesEndian;
use crate::hci::command::Command;
use crate::hci::event::{ReturnParameters, StatusReturn};
use crate::hci::le::LEControllerOpcode;
use crate::hci::{ErrorCode, Opcode};
use crate::le::advertiser::{
    AdvertisingInterval, AdvertisingParameters, AdvertisingType, ChannelMap, FilterPolicy,
    OwnAddressType, PeerAddressType,
};
use crate::ConversionError;
use crate::{BTAddress, PackError, BT_ADDRESS_LEN, RSSI};
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
        Self::COMMAND_BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(Self::COMMAND_BYTE_LEN, buf)?;
        buf[0] = self.len;
        let l = usize::from(self.len);
        buf[1..][..l].copy_from_slice(&self.data[..l]);
        // Zero the rest of the bytes in the advertisement
        buf[1..][l..].iter_mut().for_each(|i| *i = 0);
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
    const COMMAND_BYTE_LEN: usize = ADVERTISING_DATA_MAX_LEN + 1;
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
#[derive(Copy, Clone, Debug)]
pub struct SetAdvertisingParameters(pub AdvertisingParameters);
impl Command for SetAdvertisingParameters {
    type Return = StatusReturn;

    fn opcode() -> Opcode {
        LEControllerOpcode::SetAdvertisingParameters.into()
    }

    fn byte_len(&self) -> usize {
        AdvertisingParameters::BYTE_LEN
    }

    fn pack_into(&self, buf: &mut [u8]) -> Result<(), PackError> {
        PackError::expect_length(AdvertisingParameters::BYTE_LEN, buf)?;
        buf[0..2].copy_from_slice(&u16::from(self.0.interval_min).to_bytes_le()[..]);
        buf[2..4].copy_from_slice(&u16::from(self.0.interval_max).to_bytes_le()[..]);
        buf[4] = self.0.advertising_type.into();
        buf[5] = self.0.own_address_type.into();
        buf[6] = self.0.peer_address_type.into();
        self.0
            .peer_address
            .pack_into(&mut buf[7..7 + BT_ADDRESS_LEN])?;
        buf[7 + BT_ADDRESS_LEN] = self.0.channel_map.into();
        buf[8 + BT_ADDRESS_LEN] = self.0.filter_policy.into();
        Ok(())
    }

    fn unpack_from(buf: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        PackError::expect_length(AdvertisingParameters::BYTE_LEN, buf)?;
        Ok(SetAdvertisingParameters(AdvertisingParameters {
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
        }))
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

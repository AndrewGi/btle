pub mod central;

use crate::ConversionError;
use core::convert::TryFrom;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct MTU(u16);
impl MTU {
    pub const DEFAULT_U16: u16 = 23;
    pub const DEFAULT: MTU = MTU(Self::DEFAULT_U16);
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x0000;
    pub const MIN: MTU = MTU(Self::MIN_U16);
    pub const MAX_U16: u16 = 512;
    pub const MAX: MTU = MTU(Self::MAX_U16);
    pub fn new(value: u16) -> Self {
        match Self::new_checked(value) {
            Some(s) => s,
            None => panic!("MTU out of range (`{}`)", value),
        }
    }
    pub fn new_checked(value: u16) -> Option<Self> {
        if value > Self::MAX_U16 || value < Self::MIN_U16 {
            None
        } else {
            Some(Self(value))
        }
    }
}
impl From<MTU> for u16 {
    fn from(m: MTU) -> Self {
        m.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ConnectionInterval(u16);
impl ConnectionInterval {
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x0006;
    pub const MIN: ConnectionInterval = ConnectionInterval(Self::MIN_U16);
    pub const MAX_U16: u16 = 0x0C80;
    pub const MAX: ConnectionInterval = ConnectionInterval(Self::MAX_U16);
    pub fn new(value: u16) -> Self {
        match Self::new_checked(value) {
            Some(s) => s,
            None => panic!("connection interval out of range (`{}`)", value),
        }
    }
    pub fn new_checked(value: u16) -> Option<Self> {
        if value > Self::MAX_U16 || value < Self::MIN_U16 {
            None
        } else {
            Some(Self(value))
        }
    }
}
impl From<ConnectionInterval> for u16 {
    fn from(i: ConnectionInterval) -> Self {
        i.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct SupervisionTimeout(u16);
impl SupervisionTimeout {
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x000A;
    pub const MIN: SupervisionTimeout = SupervisionTimeout(Self::MIN_U16);
    pub const MAX_U16: u16 = 0x0C80;
    pub const MAX: SupervisionTimeout = SupervisionTimeout(Self::MAX_U16);
    pub fn new(value: u16) -> Self {
        match Self::new_checked(value) {
            Some(s) => s,
            None => panic!("supervision timeout out of range (`{}`)", value),
        }
    }
    pub fn new_checked(value: u16) -> Option<Self> {
        if value > Self::MAX_U16 || value < Self::MIN_U16 {
            None
        } else {
            Some(Self(value))
        }
    }
}
impl From<SupervisionTimeout> for u16 {
    fn from(t: SupervisionTimeout) -> Self {
        t.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct CELength(pub u16);
impl CELength {
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x0000;
    pub const MIN: CELength = CELength(Self::MIN_U16);
    pub const MAX_U16: u16 = 0xFFFF;
    pub const MAX: CELength = CELength(Self::MAX_U16);
}
impl From<CELength> for u16 {
    fn from(l: CELength) -> Self {
        l.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ConnectionLatency(u16);
impl ConnectionLatency {
    pub fn new(value: u16) -> Self {
        match Self::new_checked(value) {
            Some(s) => s,
            None => panic!("connection latency out of range (`{}`)", value),
        }
    }
    pub fn new_checked(value: u16) -> Option<Self> {
        if value > Self::MAX_U16 || value < Self::MIN_U16 {
            None
        } else {
            Some(Self(value))
        }
    }
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x0000;
    pub const MIN: ConnectionLatency = ConnectionLatency(Self::MIN_U16);
    pub const MAX_U16: u16 = 0x01F3;
    pub const MAX: ConnectionLatency = ConnectionLatency(Self::MAX_U16);
}
impl From<ConnectionLatency> for u16 {
    fn from(l: ConnectionLatency) -> Self {
        l.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct ConnectionHandle(u16);
impl ConnectionHandle {
    pub fn new(value: u16) -> Self {
        match Self::new_checked(value) {
            Some(s) => s,
            None => panic!("connection handle out of range (`{}`)", value),
        }
    }
    pub fn new_checked(value: u16) -> Option<Self> {
        if value > Self::MAX_U16 || value < Self::MIN_U16 {
            None
        } else {
            Some(Self(value))
        }
    }
    pub const BYTE_LEN: usize = 2;
    pub const MIN_U16: u16 = 0x0000;
    pub const MIN: ConnectionHandle = ConnectionHandle(Self::MIN_U16);
    pub const MAX_U16: u16 = 0x0EFF;
    pub const MAX: ConnectionHandle = ConnectionHandle(Self::MAX_U16);
}

impl From<ConnectionHandle> for u16 {
    fn from(h: ConnectionHandle) -> Self {
        h.0
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum InitiatorFilterPolicy {
    PeerAddress = 0x00,
    WhiteList = 0x01,
}
impl InitiatorFilterPolicy {
    pub const BYTE_LEN: usize = 1;
}
impl From<InitiatorFilterPolicy> for u8 {
    fn from(p: InitiatorFilterPolicy) -> Self {
        p as u8
    }
}
impl TryFrom<u8> for InitiatorFilterPolicy {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(InitiatorFilterPolicy::PeerAddress),
            0x01 => Ok(InitiatorFilterPolicy::WhiteList),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum Role {
    Master = 0x00,
    Slave = 0x01,
}
impl Role {
    pub const BYTE_LEN: usize = 1;
}
impl From<Role> for u8 {
    fn from(r: Role) -> Self {
        r as u8
    }
}
impl TryFrom<u8> for Role {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Role::Master),
            0x01 => Ok(Role::Slave),
            _ => Err(ConversionError(())),
        }
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum MasterClockAccuracy {
    PPM500 = 0x00,
    PPM250 = 0x01,
    PPM150 = 0x02,
    PPM100 = 0x03,
    PPM75 = 0x04,
    PPM50 = 0x05,
    PPM30 = 0x06,
    PPM20 = 0x07,
}
impl MasterClockAccuracy {
    pub const BYTE_LEN: usize = 1;
    /// Returns the clock PPM as a `u16`
    /// # Example
    /// ```
    /// use btle::le::connection::MasterClockAccuracy;
    /// assert_eq!(MasterClockAccuracy::PPM20.ppm(), 20_u16);
    /// assert_eq!(MasterClockAccuracy::PPM500.ppm(), 500_u16);
    /// ```
    pub fn ppm(self) -> u16 {
        match self {
            MasterClockAccuracy::PPM500 => 500,
            MasterClockAccuracy::PPM250 => 250,
            MasterClockAccuracy::PPM150 => 150,
            MasterClockAccuracy::PPM100 => 100,
            MasterClockAccuracy::PPM75 => 75,
            MasterClockAccuracy::PPM50 => 50,
            MasterClockAccuracy::PPM30 => 30,
            MasterClockAccuracy::PPM20 => 20,
        }
    }
}
impl From<MasterClockAccuracy> for u8 {
    fn from(m: MasterClockAccuracy) -> Self {
        m as u8
    }
}
impl TryFrom<u8> for MasterClockAccuracy {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(MasterClockAccuracy::PPM500),
            0x01 => Ok(MasterClockAccuracy::PPM250),
            0x02 => Ok(MasterClockAccuracy::PPM150),
            0x03 => Ok(MasterClockAccuracy::PPM100),
            0x04 => Ok(MasterClockAccuracy::PPM75),
            0x05 => Ok(MasterClockAccuracy::PPM50),
            0x06 => Ok(MasterClockAccuracy::PPM30),
            0x07 => Ok(MasterClockAccuracy::PPM20),
            _ => Err(ConversionError(())),
        }
    }
}

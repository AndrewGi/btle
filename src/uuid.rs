//! Universally Unique Identifiers or UUIDs.
use crate::ConversionError;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::{Display, Error, Formatter};

type UUIDBytes = [u8; 16];

#[cfg_attr(feature = "serde-1", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct UUID(pub UUIDBytes);

impl UUID {
    // TODO: Write new UUID functions (versions 1-4)
    #[must_use]
    pub fn from_fields(
        time_low: u32,
        time_mid: u16,
        time_high: u16,
        clock_seq: u16,
        node: u64,
    ) -> UUID {
        let tl = time_low.to_le_bytes();
        let tm = time_mid.to_le_bytes();
        let th = time_high.to_le_bytes();
        let cs = clock_seq.to_le_bytes();
        let nb = node.to_le_bytes();
        // This is a dumb way of building a UUID from byte slices but it should work.
        UUID([
            tl[0], tl[1], tl[2], tl[3], tm[0], tm[1], th[0], th[1], cs[0], cs[1], nb[0], nb[1],
            nb[2], nb[3], nb[4], nb[5],
        ])
    }
    #[must_use]
    pub fn time_low(&self) -> u32 {
        u32::from_le_bytes([self.0[0], self.0[1], self.0[2], self.0[3]])
    }
    #[must_use]
    pub fn time_mid(&self) -> u16 {
        u16::from_le_bytes([self.0[4], self.0[5]])
    }
    #[must_use]
    pub fn time_high(&self) -> u16 {
        u16::from_le_bytes([self.0[6], self.0[7]])
    }
    #[must_use]
    pub fn clock_seq(&self) -> u16 {
        u16::from_le_bytes([self.0[8], self.0[9]])
    }
    #[must_use]
    pub fn node(&self) -> u64 {
        u64::from_le_bytes([
            self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15], 0, 0,
        ])
    }
    /// Converts a 32-character hex string (`70cf7c9732a345b691494810d2e9cbf4`) to `UUIDBytes`.
    #[must_use]
    pub fn uuid_bytes_from_str(s: &str) -> Option<UUIDBytes> {
        let mut out = [0_u8; 16];
        let buf = out.as_mut();
        if buf.len() == 0 || buf.len() * 2 != s.len() {
            return None;
        }
        for (i, c) in s.chars().enumerate() {
            let v = u8::try_from(c.to_digit(16)?).expect("only returns [0..=15]");
            buf[i / 2] |= v << u8::try_from(((i + 1) % 2) * 4).expect("only returns 0 or 4");
        }
        Some(out)
    }
}
impl TryFrom<&[u8]> for UUID {
    type Error = ConversionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(UUID(value.try_into().map_err(|_| ConversionError(()))?))
    }
}
impl AsRef<[u8]> for UUID {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl AsMut<[u8]> for UUID {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UUIDFields {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_high: u16,
    pub clock_seq: u16,
    pub node: u64,
}

impl Into<UUIDFields> for &UUID {
    #[must_use]
    fn into(self) -> UUIDFields {
        UUIDFields {
            time_low: self.time_low(),
            time_mid: self.time_mid(),
            time_high: self.time_high(),
            clock_seq: self.clock_seq(),
            node: self.node(),
        }
    }
}
impl Into<UUIDFields> for UUID {
    fn into(self) -> UUIDFields {
        (&self).into()
    }
}
impl Into<UUID> for &UUIDFields {
    #[must_use]
    fn into(self) -> UUID {
        UUID::from_fields(
            self.time_low,
            self.time_mid,
            self.time_high,
            self.clock_seq,
            self.node,
        )
    }
}
impl Into<UUID> for UUIDFields {
    #[must_use]
    fn into(self) -> UUID {
        (&self).into()
    }
}
impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.time_low(),
            self.time_mid(),
            self.time_high(),
            self.clock_seq(),
            self.node()
        )
    }
}
/// 16-bit UUID
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct UUID16(pub u16);
impl UUID16 {
    pub const fn new(uuid_short: u16) -> UUID16 {
        UUID16(uuid_short)
    }
}
impl From<UUID16> for u16 {
    fn from(u: UUID16) -> Self {
        u.0
    }
}
/// 32-bit UUID
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct UUID32(pub u32);
impl UUID32 {
    pub const fn new(uuid_short: u32) -> UUID32 {
        UUID32(uuid_short)
    }
}
impl From<UUID32> for u32 {
    fn from(u: UUID32) -> Self {
        u.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format() {
        let time_low = 0x123e4567;
        let time_mid = 0xe89b;
        let time_high = 0x12d3;
        let clock_seq = 0xa456;
        let node = 0x426655440000;
        let uuid = UUID::from_fields(time_low, time_mid, time_high, clock_seq, node);

        let fields: UUIDFields = uuid.into();
        assert_eq!(time_low, fields.time_low);
        assert_eq!(time_mid, fields.time_mid);
        assert_eq!(time_high, fields.time_high);
        assert_eq!(clock_seq, fields.clock_seq);
        assert_eq!(node, fields.node);

        assert_eq!(uuid.to_string(), "123e4567-e89b-12d3-a456-426655440000");
    }
}

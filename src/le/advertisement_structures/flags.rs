use crate::le::advertisement::AdType;

pub enum BitFlags {
    LELimitedDiscoverableMode,
    LEGeneralDiscoverableMode,
    BrEdrNotSupported,
    SimultaneousLEAndBrEdrController,
    SimultaneousLEAndBrEdrHost,
}
pub struct Flags(u8);
impl Flags {
    pub const AD_TYPE: AdType = AdType::Flags;
}

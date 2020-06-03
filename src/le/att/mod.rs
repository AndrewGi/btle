use crate::ConversionError;

pub mod attribute;
pub mod authentication;
pub mod error;
pub mod pdus;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum Opcode {
    ErrorRsp = 0x01,
    ExchangeMTUReq = 0x02,
    ExchangeMTURsp = 0x03,
    FindInformationReq = 0x04,
    FindInformationRsp = 0x05,
    FindByTypeValueReq = 0x06,
    FindByTypeValueRsp = 0x07,
    FindByTypeReq = 0x08,
    FindByTypeRsp = 0x09,
    ReadReq = 0x0A,
    ReadRsp = 0x0B,
    ReadBlobReq = 0x0C,
    ReadBlobRsp = 0x0D,
    ReadMultipleReq = 0x0E,
    ReadMultipleRsp = 0x0F,
    ReadByGroupTypeReq = 0x10,
    ReadByGroupTypeRsp = 0x11,
    WriteReq = 0x12,
    WriteRsp = 0x13,
    // 0x14 SKIPPED
    // 0x15 SKIPPED
    PrepareWriteReq = 0x16,
    PrepareWriteRsp = 0x17,
    ExecuteWriteReq = 0x18,
    ExecuteWriteRsp = 0x19,
    // 0x1A SKIPPED
    HandleValueNtf = 0x1B,
    // 0x1C SKIPPED
    HandleValueInd = 0x1D,
    HandleValueCfm = 0x1E,
    // 0x1F SKIPPED
    ReadMultipleVariableReq = 0x20,
    ReadMultipleVariableRsp = 0x21,
    // 0x22 SKIPPED
    MultipleHandleValueNtf = 0x23,

    WriteCmd = 0x52,
    SignedWriteCmd = 0xD2,
}
impl From<Opcode> for u8 {
    fn from(o: Opcode) -> Self {
        o as u8
    }
}
impl core::convert::TryFrom<u8> for Opcode {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, ConversionError> {
        match value {
            0x01 => Ok(Opcode::ErrorRsp),
            0x02 => Ok(Opcode::ExchangeMTUReq),
            0x03 => Ok(Opcode::ExchangeMTURsp),
            0x04 => Ok(Opcode::FindInformationReq),
            0x05 => Ok(Opcode::FindInformationRsp),
            0x06 => Ok(Opcode::FindByTypeValueReq),
            0x07 => Ok(Opcode::FindByTypeValueRsp),
            0x08 => Ok(Opcode::FindByTypeReq),
            0x09 => Ok(Opcode::FindByTypeRsp),
            0x0A => Ok(Opcode::ReadReq),
            0x0B => Ok(Opcode::ReadRsp),
            0x0C => Ok(Opcode::ReadBlobReq),
            0x0D => Ok(Opcode::ReadBlobRsp),
            0x0E => Ok(Opcode::ReadMultipleReq),
            0x0F => Ok(Opcode::ReadMultipleRsp),
            0x10 => Ok(Opcode::ReadByGroupTypeReq),
            0x11 => Ok(Opcode::ReadByGroupTypeRsp),
            0x12 => Ok(Opcode::WriteReq),
            0x13 => Ok(Opcode::WriteRsp),
            0x16 => Ok(Opcode::PrepareWriteReq),
            0x17 => Ok(Opcode::PrepareWriteRsp),
            0x18 => Ok(Opcode::ExecuteWriteReq),
            0x19 => Ok(Opcode::ExecuteWriteRsp),
            0x1B => Ok(Opcode::HandleValueNtf),
            0x1D => Ok(Opcode::HandleValueInd),
            0x1E => Ok(Opcode::HandleValueCfm),
            0x20 => Ok(Opcode::ReadMultipleVariableReq),
            0x21 => Ok(Opcode::ReadMultipleVariableRsp),
            0x23 => Ok(Opcode::MultipleHandleValueNtf),
            0x52 => Ok(Opcode::WriteCmd),
            0xD2 => Ok(Opcode::SignedWriteCmd),
            _ => Err(ConversionError(())),
        }
    }
}
impl Opcode {
    pub const BYTE_LEN: usize = 1;
}

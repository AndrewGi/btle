use crate::uuid::UUID;
use crate::windows::{guid_to_uuid, WindowsError};
use windows::{
    Devices::Bluetooth::GenericAttributeProfile::GattLocalDescriptor,
    Storage::Streams::DataReader
};


pub struct LocalDescriptor(GattLocalDescriptor);

impl LocalDescriptor {
    pub const fn from_inner(inner: GattLocalDescriptor) -> Self {
        Self(inner)
    }
    pub fn into_inner(self) -> GattLocalDescriptor {
        self.0
    }

    pub fn uuid(&self) -> Result<UUID, WindowsError> {
        Ok(guid_to_uuid(&self.0.Uuid()?))
    }
    pub fn static_value(&self) -> Result<Vec<u8>, WindowsError> {
        let buf = self.0.StaticValue()?;
        let reader = DataReader::FromBuffer(&buf)?;
        let len = reader.UnconsumedBufferLength()? as usize;
        let mut out = vec![0_u8; len];
        reader.ReadBytes(out.as_mut_slice())?;
        Ok(out)
    }
}

use crate::uuid::UUID;
use crate::windows::{guid_to_uuid, WindowsError};
use winrt_bluetooth_bindings::windows::devices::bluetooth::generic_attribute_profile::GattLocalDescriptor;
use winrt_bluetooth_bindings::windows::storage::streams::DataReader;

pub struct LocalDescriptor(GattLocalDescriptor);

impl LocalDescriptor {
    pub const fn from_inner(inner: GattLocalDescriptor) -> Self {
        Self(inner)
    }
    pub fn into_inner(self) -> GattLocalDescriptor {
        self.0
    }

    pub fn uuid(&self) -> Result<UUID, WindowsError> {
        Ok(guid_to_uuid(&self.0.uuid()?))
    }
    pub fn static_value(&self) -> Result<Vec<u8>, WindowsError> {
        let buf = self.0.static_value()?;
        let reader = DataReader::from_buffer(buf)?;
        let len = reader.unconsumed_buffer_length()? as usize;
        let mut out = vec![0_u8; len];
        reader.read_bytes(out.as_mut_slice())?;
        Ok(out)
    }
}

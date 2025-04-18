use crate::windows::WindowsError;
use windows::{
    Storage::Streams::{DataReader, DataWriter},
    Devices::Bluetooth::GenericAttributeProfile::{
        GattCharacteristicProperties, GattLocalCharacteristic, GattLocalCharacteristicParameters,
    }
};

/*
pub struct LocalCharacteristics(IVectorView<GattLocalCharacteristic>);
impl LocalCharacteristics {
    pub fn size(&self) -> Result<usize, WindowsError> {
        Ok(self.0.size()? as usize)
    }
    pub fn get_at(&self, index: usize) -> Result<LocalCharacteristic, WindowsError> {
        Ok(LocalCharacteristic(self.0.get_at(index as u32)?))
    }
    pub fn iter(
        &self,
    ) -> Result<impl Iterator<Item = Result<LocalCharacteristic, WindowsError>> + '_, WindowsError>
    {
        Ok(LocalCharacteristicsIter {
            index: 0,
            len: self.size()?,
            characteristics: self,
        })
    }
}
struct LocalCharacteristicsIter<'a> {
    index: usize,
    len: usize,
    characteristics: &'a LocalCharacteristics,
}
impl<'a> Iterator for LocalCharacteristicsIter<'a> {
    type Item = Result<LocalCharacteristic, WindowsError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            match self.characteristics.get_at(self.index) {
                Ok(out) => {
                    self.index += 1;
                    Some(Ok(out))
                }
                Err(e) => {
                    // Stop the iterator on error
                    self.index = self.len;
                    Some(Err(e))
                }
            }
        }
    }
}
    */
pub struct LocalCharacteristic(GattLocalCharacteristic);
impl LocalCharacteristic {
    pub fn from_inner(inner: GattLocalCharacteristic) -> Self {
        Self(inner)
    }
    pub fn into_inner(self) -> GattLocalCharacteristic {
        self.0
    }
}

pub struct LocalCharacteristicParameters(GattLocalCharacteristicParameters);
impl LocalCharacteristicParameters {
    pub fn new() -> Result<Self, WindowsError> {
        Ok(Self(GattLocalCharacteristicParameters::new()?))
    }
    pub fn into_inner(self) -> GattLocalCharacteristicParameters {
        self.0
    }
    pub fn static_value(&self) -> Result<Vec<u8>, WindowsError> {
        let buf = self.0.StaticValue()?;
        let reader = DataReader::FromBuffer(&buf)?;
        let len = reader.UnconsumedBufferLength()? as usize;
        let mut out = vec![0_u8; len];
        reader.ReadBytes(out.as_mut_slice())?;
        Ok(out)
    }
    pub fn set_static_value(&self, value: &[u8]) -> Result<(), WindowsError> {
        let writer = DataWriter::new()?;
        writer.WriteBytes(value)?;
        self.0.SetStaticValue(&writer.DetachBuffer()?)?;
        Ok(())
    }
    pub fn user_description(&self) -> Result<String, WindowsError> {
        Ok(self.0.UserDescription()?.to_string_lossy())
    }
    pub fn set_user_description(&self, description: &str) -> Result<(), WindowsError> {
        self.0
            .SetUserDescription(&windows::core::HSTRING::from(description))?;
        Ok(())
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CharacteristicProperty {
    Broadcast,
    Read,
    Write,
    WriteWithoutResponse,
    Notify,
    Indicate,
    AuthenticatedSignedWrites,
    ExtendedProperties,
    ReliableWrites,
    WriteableAuxiliaries,
}
impl CharacteristicProperty {
    pub fn into_winrt(self) -> GattCharacteristicProperties {
        match self {
            CharacteristicProperty::Broadcast => GattCharacteristicProperties::Broadcast,
            CharacteristicProperty::Read => GattCharacteristicProperties::Read,
            CharacteristicProperty::Write => GattCharacteristicProperties::Write,
            CharacteristicProperty::WriteWithoutResponse => {
                GattCharacteristicProperties::WriteWithoutResponse
            }
            CharacteristicProperty::Notify => GattCharacteristicProperties::Notify,
            CharacteristicProperty::Indicate => GattCharacteristicProperties::Indicate,
            CharacteristicProperty::AuthenticatedSignedWrites => {
                GattCharacteristicProperties::AuthenticatedSignedWrites
            }
            CharacteristicProperty::ExtendedProperties => {
                GattCharacteristicProperties::ExtendedProperties
            }
            CharacteristicProperty::ReliableWrites => GattCharacteristicProperties::ReliableWrites,
            CharacteristicProperty::WriteableAuxiliaries => {
                GattCharacteristicProperties::WritableAuxiliaries
            }
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct CharacteristicProperties(GattCharacteristicProperties);
impl CharacteristicProperties {
    pub fn is_set(self, flag: CharacteristicProperty) -> bool {
        self.0 & flag.into_winrt() != GattCharacteristicProperties::None
    }
    pub fn set(&mut self, flag: CharacteristicProperty) {
        self.0 = self.0 | flag.into_winrt();
    }
    pub fn clear(&mut self, flag: CharacteristicProperty) {
        // SAFETY:
        // `GattCharacteristicProperties` doesn't implement bit-wise not so we do it manually
        let inverted = unsafe {
            std::mem::transmute::<u32, GattCharacteristicProperties>(!std::mem::transmute::<
                GattCharacteristicProperties,
                u32,
            >(
                flag.into_winrt()
            ))
        };
        self.0 = self.0 & inverted;
    }
    pub fn is_none(&self) -> bool {
        self.0 == GattCharacteristicProperties::None
    }
}

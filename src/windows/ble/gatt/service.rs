use crate::uuid::UUID;
use crate::windows::ble::gatt::characteristic::{
    LocalCharacteristic, LocalCharacteristicParameters,
};
use crate::windows::{guid_to_uuid, uuid_to_guid, BluetoothError, WindowsError};
use winrt_bluetooth_bindings::windows::devices::bluetooth::generic_attribute_profile::{
    GattDeviceServicesResult, GattLocalService, GattServiceProvider,
    GattServiceProviderAdvertisementStatus,
};

pub struct ServicesResult(GattDeviceServicesResult);
impl ServicesResult {
    pub fn from_inner(inner: GattDeviceServicesResult) -> Self {
        Self(inner)
    }
    pub fn protocol_error(&self) -> Result<u8, WindowsError> {
        Ok(self.0.protocol_error()?.value()?)
    }
}

pub struct ServiceProvider(GattServiceProvider);
impl ServiceProvider {
    pub async fn new(uuid: &UUID) -> Result<Self, WindowsError> {
        Ok(Self(
            GattServiceProvider::create_async(uuid_to_guid(uuid))?
                .await?
                .service_provider()?,
        ))
    }
    pub fn start_advertising(&self) -> Result<(), WindowsError> {
        Ok(self.0.start_advertising()?)
    }
    pub fn stop_advertising(&self) -> Result<(), WindowsError> {
        Ok(self.0.stop_advertising()?)
    }
    pub fn service(&self) -> Result<LocalService, WindowsError> {
        Ok(LocalService(self.0.service()?))
    }
    pub fn advertisement_status(&self) -> Result<AdvertisementStatus, WindowsError> {
        Ok(self.0.advertisement_status()?.into())
    }
}

pub struct LocalService(GattLocalService);
impl LocalService {
    pub fn uuid(&self) -> Result<UUID, WindowsError> {
        Ok(guid_to_uuid(&self.0.uuid()?))
    }

    pub async fn create_characteristic(
        &self,
        uuid: &UUID,
        parameters: LocalCharacteristicParameters,
    ) -> Result<Result<LocalCharacteristic, BluetoothError>, WindowsError> {
        let result = self
            .0
            .create_characteristic_async(uuid_to_guid(uuid), parameters.into_inner())?
            .await?;
        match BluetoothError::from(result.error()?) {
            BluetoothError::Success => (),
            e => return Ok(Err(e)),
        };
        Ok(Ok(LocalCharacteristic::from_inner(
            result.characteristic()?,
        )))
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum AdvertisementStatus {
    Created,
    Stopped,
    Started,
    Aborted,
    StartedWithoutAllAdvertisementData,
}
impl AdvertisementStatus {
    pub fn to_winrt(self) -> GattServiceProviderAdvertisementStatus {
        match self {
            AdvertisementStatus::Created => GattServiceProviderAdvertisementStatus::Created,
            AdvertisementStatus::Stopped => GattServiceProviderAdvertisementStatus::Stopped,
            AdvertisementStatus::Started => GattServiceProviderAdvertisementStatus::Started,
            AdvertisementStatus::Aborted => GattServiceProviderAdvertisementStatus::Aborted,
            AdvertisementStatus::StartedWithoutAllAdvertisementData => {
                GattServiceProviderAdvertisementStatus::StartedWithoutAllAdvertisementData
            }
        }
    }
    pub fn from_winrt(i: GattServiceProviderAdvertisementStatus) -> Self {
        match i {
            GattServiceProviderAdvertisementStatus::Created => Self::Created,
            GattServiceProviderAdvertisementStatus::Stopped => Self::Stopped,
            GattServiceProviderAdvertisementStatus::Started => Self::Started,
            GattServiceProviderAdvertisementStatus::Aborted => Self::Aborted,
            GattServiceProviderAdvertisementStatus::StartedWithoutAllAdvertisementData => {
                Self::StartedWithoutAllAdvertisementData
            }
            _ => panic!("invalid GattServiceProviderAdvertisementStatus state"),
        }
    }
}
impl From<GattServiceProviderAdvertisementStatus> for AdvertisementStatus {
    fn from(i: GattServiceProviderAdvertisementStatus) -> Self {
        Self::from_winrt(i)
    }
}
impl From<AdvertisementStatus> for GattServiceProviderAdvertisementStatus {
    fn from(i: AdvertisementStatus) -> Self {
        i.to_winrt()
    }
}

use crate::hci::adapters;
use crate::hci::le;
use crate::hci::le::random::RAND_LEN;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter};
use core::pin::Pin;

pub struct LEAdapter<'a, S: HCIWriter + HCIReader + HCIFilterable> {
    adapter: Pin<&'a mut adapters::Adapter<S>>,
}
impl<'a, S: HCIWriter + HCIReader + HCIFilterable> LEAdapter<'a, S> {
    pub fn new(adapter: Pin<&'a mut adapters::Adapter<S>>) -> Self {
        Self { adapter }
    }
    pub fn adapter_mut(&mut self) -> Pin<&mut adapters::Adapter<S>> {
        self.adapter.as_mut()
    }
    pub fn adapter_ref(&self) -> Pin<&adapters::Adapter<S>> {
        self.adapter.as_ref()
    }
    pub async fn set_scan_enabled(
        &mut self,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> Result<(), adapters::Error> {
        self.adapter_mut()
            .send_command(le::SetScanEnable {
                is_enabled,
                filter_duplicates,
            })
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    pub async fn set_scan_parameters(
        &mut self,
        scan_parameters: le::SetScanParameters,
    ) -> Result<(), adapters::Error> {
        self.adapter_mut()
            .send_command(scan_parameters)
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    pub async fn set_advertising_enabled(
        &mut self,
        is_enabled: bool,
    ) -> Result<(), adapters::Error> {
        self.adapter_mut()
            .send_command(le::SetAdvertisingEnable { is_enabled })
            .await?
            .params
            .status
            .error()?;
        Ok(())
    }
    /// Get `RAND_LEN` (8) bytes from the HCI Controller.
    pub async fn get_rand(&mut self) -> Result<[u8; RAND_LEN], adapters::Error> {
        let r = self.adapter_mut().send_command(le::Rand {}).await?;
        r.params.status.error()?;
        Ok(r.params.random_bytes)
    }
}

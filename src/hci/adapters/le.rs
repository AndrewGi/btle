use crate::bytes::Storage;
use crate::hci::adapters;
use crate::hci::event::{CommandComplete, StatusReturn};
use crate::hci::le;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter};
use core::pin::Pin;

pub struct LEAdapter<'a, S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> {
    adapter: Pin<&'a mut adapters::Adapter<S, Buf>>,
}
impl<'a, S: HCIWriter + HCIReader + HCIFilterable, Buf: Storage> LEAdapter<'a, S, Buf> {
    pub fn new(adapter: Pin<&'a mut adapters::Adapter<S, Buf>>) -> Self {
        Self { adapter }
    }
    pub fn adapter_mut(&mut self) -> Pin<&mut adapters::Adapter<S, Buf>> {
        self.adapter.as_mut()
    }
    pub fn adapter_ref(&self) -> Pin<&adapters::Adapter<S, Buf>> {
        self.adapter.as_ref()
    }
    pub async fn set_scan_enabled(
        &mut self,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> Result<CommandComplete<StatusReturn>, adapters::Error> {
        self.adapter_mut()
            .send_command(le::SetScanEnable {
                is_enabled,
                filter_duplicates,
            })
            .await
    }
    pub async fn set_advertising_enabled(
        &mut self,
        is_enabled: bool,
    ) -> Result<CommandComplete<StatusReturn>, adapters::Error> {
        self.adapter_mut()
            .send_command(le::SetAdvertisingEnable { is_enabled })
            .await
    }
}

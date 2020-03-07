use crate::bytes::Storage;
use crate::hci::adapters;
use crate::hci::adapters::Adapter;
use crate::hci::command::Command;
use crate::hci::event::StatusReturn;
use crate::hci::le;
use crate::hci::stream::{HCIFilterable, HCIReader, HCIWriter};
use core::pin::Pin;

pub struct LEAdapter<'a, S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage> {
    adapter: &'a mut adapters::Adapter<S, Buf>,
}
impl<'a, S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage> LEAdapter<'a, S, Buf> {
    pub fn new(adapter: &'a mut adapters::Adapter<S, Buf>) -> Self {
        Self { adapter }
    }
    fn pinned_adapter(self: Pin<&mut Self>) -> Pin<&mut adapters::Adapter<S, Buf>> {
        unsafe { self.map_unchecked_mut(|s| s.adapter) }
    }
    pub async fn set_scan_enabled(
        self: Pin<&mut Self>,
        is_enabled: bool,
        filter_duplicates: bool,
    ) -> Result<StatusReturn, adapters::Error> {
        self.pinned_adapter()
            .send_command::<le::SetScanEnable, <le::SetScanEnable as Command>::Return>(
                le::SetScanEnable {
                    is_enabled,
                    filter_duplicates,
                },
            )
            .await
    }
    pub async fn set_advertising_enabled(
        self: Pin<&mut Self>,
        is_enabled: bool,
    ) -> Result<StatusReturn, adapters::Error> {
        self.pinned_adapter().send_command::<le::SetAdvertisingEnable, <le::SetAdvertisingEnable as Command>::Return>()
    }
}
impl<'a, S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage>
    AsRef<adapters::Adapter<S, Buf>> for LEAdapter<'a, S, Buf>
{
    fn as_ref(&self) -> &Adapter<S, Buf> {
        &self.adapter
    }
}
impl<'a, S: HCIWriter + HCIReader<Buf> + HCIFilterable, Buf: Storage>
    AsMut<adapters::Adapter<S, Buf>> for LEAdapter<'a, S, Buf>
{
    fn as_mut(&mut self) -> &mut Adapter<S, Buf> {
        &mut self.adapter
    }
}

use crate::dbus::adapter::Adapter;
use crate::dbus::DbusError;
use std::sync::Arc;
// TODO: switch by to `dbus-async`?
#[derive(Clone, Debug)]
pub struct Session(Arc<dbus_async::DBus>);
impl Session {
    pub async fn new() -> Result<Self, DbusError> {
        Ok(Self(Arc::new(dbus_async::DBus::system(false)?)))
    }
    pub async fn adapters(&self) -> Result<Vec<Adapter>, DbusError> {
        self.0.send_with_reply()
    }
}

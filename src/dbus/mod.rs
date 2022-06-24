pub mod adapter;
mod messages;
pub mod session;
#[derive(Debug)]
pub struct DbusError(pub dbus_async::DBusError);
impl From<DbusError> for dbus_async::DBusError {
    fn from(e: DbusError) -> Self {
        e.0
    }
}

impl From<dbus_async::DBusError> for DbusError {
    fn from(e: dbus_async::DBusError) -> Self {
        Self(e)
    }
}

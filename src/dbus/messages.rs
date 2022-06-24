use dbus_message_parser::{MessageFlags, MessageHeader, MessageHeaderField, MessageType};
use std::collections::BTreeSet;
use std::convert::TryInto;

static ADAPTER_INTERFACE: &'static str = "org.bluez.Adapter1";
static DEVICE_INTERFACE: &'static str = "org.bluez.Device1";
static SERVICE_INTERFACE: &'static str = "org.bluez.GattService1";
static CHARACTERISTIC_INTERFACE: &'static str = "org.bluez.GattCharacteristic1";
static DESCRIPTOR_INTERFACE: &'static str = "org.bluez.GattDescriptor1";
static SERVICE_NAME: &'static str = "org.bluez";
const IS_LE: bool = true;
fn message_call(
    destination: impl TryInto<dbus_message_parser::Bus>,
    object_path: impl TryInto<dbus_message_parser::ObjectPath>,
    interface: impl TryInto<dbus_message_parser::Interface>,
    member: impl TryInfo<dbus_message_parser::Member>,
) -> MessageHeader {
    let mut fields = BTreeSet::new();
    fields.insert(MessageHeaderField::Destination(destination));
    fields.insert(MessageHeaderField::Path(object_path));
    fields.insert(MessageHeaderField::Interface(interface));
    fields.insert(MessageHeaderField::Member(member));
    MessageHeader::new(
        IS_LE,
        MessageType::MethodCall,
        MessageFlags::empty(),
        1,
        0,
        fields,
    )
    .expect("hard coded fields")
}
pub fn get_managed_objects_header() -> MessageHeader {
    message_call(
        SERVICE_NAME,
        "/org/bluez",
        "org.freedesktop.DBus.ObjectManager",
        "GetManagedObjects",
    )
}

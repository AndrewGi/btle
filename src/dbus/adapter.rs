use crate::dbus::session::Session;

#[derive(Clone, Debug)]
pub struct Adapter {
    session: Session,
    object_path: String,
}
impl Adapter {
    const fn from_parts(session: Session, object_path: String) -> Self {
        Self {
            session,
            object_path,
        }
    }
}

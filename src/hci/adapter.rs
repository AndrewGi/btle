use crate::bytes::Storage;
use crate::error::IOError;
use crate::hci;
use crate::hci::command::{Command, CommandPacket};
use crate::hci::event::EventPacket;
use crate::hci::stream::HCI_EVENT_READ_TRIES;
use crate::hci::StreamError;
use futures_util::future::LocalBoxFuture;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum Error {
    BadParameter,
    IOError(IOError),
    StreamError(hci::StreamError),
    ErrorCode(hci::ErrorCode),
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "hci adapter error {:?}", self)
    }
}
impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::IOError(e)
    }
}
impl From<hci::StreamError> for Error {
    fn from(e: hci::StreamError) -> Self {
        Error::StreamError(e)
    }
}
impl From<hci::ErrorCode> for Error {
    fn from(e: hci::ErrorCode) -> Self {
        Error::ErrorCode(e)
    }
}
#[cfg(feature = "hci_usb")]
impl From<hci::usb::Error> for Error {
    fn from(e: hci::usb::Error) -> Self {
        Error::IOError(e.0)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl crate::error::Error for Error {}
///WIP HCI Adapter trait
pub trait Adapter {
    fn write_command<'s, 'p: 's>(
        &'s mut self,
        packet: CommandPacket<&'p [u8]>,
    ) -> LocalBoxFuture<'s, Result<(), Error>>;
    fn send_command<'a, 'c: 'a, Cmd: Command + 'c>(
        &'a mut self,
        command: Cmd,
    ) -> LocalBoxFuture<'a, Result<Cmd::Return, hci::adapter::Error>> {
        Box::pin(async move {
            type Buf = Box<[u8]>;
            // Pack Command
            self.write_command(
                command
                    .pack_command_packet::<Buf>()
                    .map_err(StreamError::CommandError)?
                    .as_ref(),
            )
            .await?;
            for _try_i in 0..HCI_EVENT_READ_TRIES {
                let event: EventPacket<Buf> = self.read_event::<Buf>().await?;
                if let Some(ret) =
                    Cmd::unpack_return(event.as_ref()).map_err(StreamError::EventError)?
                {
                    return Ok(ret);
                }
            }
            Err(hci::adapter::Error::StreamError(StreamError::StreamFailed))
        })
    }
    fn read_event<'s, 'p: 's, S: Storage<u8> + 'p>(
        &'s mut self,
    ) -> LocalBoxFuture<'s, Result<EventPacket<S>, Error>>;
}
pub struct DummyAdapter;
impl Adapter for DummyAdapter {
    fn write_command<'s, 'p: 's>(
        &'s mut self,
        packet: CommandPacket<&'p [u8]>,
    ) -> LocalBoxFuture<'s, Result<(), Error>> {
        unimplemented!("dummy adapter write event {:?}", packet)
    }
    fn send_command<'a, 'c: 'a, Cmd: Command + 'c>(
        &'a mut self,
        _command: Cmd,
    ) -> LocalBoxFuture<'a, Result<Cmd::Return, hci::adapter::Error>> {
        unimplemented!("dummy send {:?}", Cmd::opcode())
    }
    fn read_event<'s, 'p: 's, S: Storage<u8> + 'p>(
        &'s mut self,
    ) -> LocalBoxFuture<'s, Result<EventPacket<S>, Error>> {
        unimplemented!("dummy adapter read event")
    }
}

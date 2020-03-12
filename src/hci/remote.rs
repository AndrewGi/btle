//! Remote HCI Controller (WIP).
use std::{io, net};
pub struct Client(pub net::TcpStream);
impl io::Write for Client {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.0.flush()
    }
}
impl io::Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.0.read(buf)
    }
}
impl Client {
    pub fn new(stream: net::TcpStream) -> Self {
        Self(stream)
    }
}
#[cfg(feature = "remote_async")]
pub mod remote_async {
    use core::pin::Pin;
    use core::task::{Context, Poll};
    use tokio::io::AsyncRead;

    pub struct AsyncClient(pub tokio::net::TcpStream);
    impl futures::io::AsyncRead for AsyncClient {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<Result<usize, std::io::Error>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }
}

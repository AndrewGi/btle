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
pub mod server {
    use super::socket;
    use std::{io, net};
    pub struct Server {
        tcp: net::TcpStream,
        hci: socket::HCISocket,
    }
}

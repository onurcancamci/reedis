use std::net::TcpStream;

pub trait Peek {
    fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize>;
}

impl Peek for TcpStream {
    fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.peek(buf)
    }
}

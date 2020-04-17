use crate::*;
use std::io::{Read, Write};
use std::net::TcpStream;

enum MemType {
    Stack,
    Heap,
    Invalid,
}
pub struct SocketProvider {
    buf: [u8; BUFFER_SIZE],
    heap: Vec<u8>,
    socket: TcpStream,
    mem_type: MemType,
}

impl SocketProvider {
    pub fn new(socket: TcpStream) -> SocketProvider {
        SocketProvider {
            buf: [0; BUFFER_SIZE],
            socket,
            mem_type: MemType::Invalid,
            heap: vec![],
        }
    }
    fn read_len(&mut self) -> AppResult<usize> {
        let mut len_buf = [0; std::mem::size_of::<usize>()];
        self.socket
            .read_exact(&mut len_buf)
            .map_err(|_| AppError::SocketReadError)?;
        Ok(usize::from_le_bytes(len_buf))
    }
    fn read_bytes(&mut self, len: usize) -> AppResult<()> {
        self.socket
            .read_exact(&mut self.buf[0..len])
            .map_err(|_| AppError::SocketReadError)?;
        Ok(())
    }
    fn read_bytes_heap(&mut self, len: usize) -> AppResult<()> {
        self.heap = Vec::with_capacity(len);
        let mut buf = [0u8; 1024];
        let mut clen = 0;
        while clen < len {
            let cclen = self
                .socket
                .read(&mut buf[0..1024.min(len - clen)])
                .map_err(|_| AppError::SocketReadError)?;
            self.heap.extend_from_slice(&buf[0..cclen]);
            clen += cclen;
        }

        Ok(())
    }
    pub fn read_packet(&mut self) -> AppResult<usize> {
        let len = self.read_len()?;

        if len < BUFFER_SIZE {
            self.read_bytes(len)?;
            self.mem_type = MemType::Stack;
            Ok(len)
        } else {
            self.read_bytes_heap(len)?;
            self.mem_type = MemType::Heap;
            Ok(len)
        }
    }
    pub fn content(&self) -> AppResult<&[u8]> {
        match self.mem_type {
            MemType::Stack => Ok(&self.buf),
            MemType::Invalid => Err(AppError::SocketReadError),
            MemType::Heap => Ok(self.heap.as_slice()),
        }
    }
    pub fn write(&mut self, buf: &[u8]) -> AppResult<usize> {
        self.socket
            .write(buf)
            .map_err(|_| AppError::SocketWriteError)
    }
}

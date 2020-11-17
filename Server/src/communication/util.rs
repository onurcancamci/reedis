use std::io::Read;

pub enum Message {
    Stack([u8; 1024]),
    Heap(Vec<u8>),
}

impl Message {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Message::Stack(s) => s,
            Message::Heap(h) => h.as_slice(),
        }
    }
}

pub fn read_message<T>(stream: &mut T) -> Result<Message, Box<dyn std::error::Error>>
where
    T: Read,
{
    let mut len = [0u8; 4];
    stream.read_exact(&mut len)?;
    let len = u32::from_le_bytes(len);

    if len > 1024 {
        let mut vec = vec![0u8; len as usize];
        stream.read_exact(vec.as_mut_slice())?;
        Ok(Message::Heap(vec))
    } else {
        let mut buf = [0u8; 1024];
        stream.read_exact(&mut buf)?;
        Ok(Message::Stack(buf))
    }
}

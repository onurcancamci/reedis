use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

use crate::*;

pub fn handle_client(
    mut stream: TcpStream,
    tx_register: Sender<(usize, Sender<Event>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // wait for intent, data / event
    let mut intent = [0u8; 1];
    stream.read_exact(&mut intent)?;

    match intent[0] {
        0u8 => {
            // version 0, data, language = json
            handle_data(stream)?;
        }
        1u8 => {
            // version 0, event, language = json
            handle_event(stream, tx_register)?;
        }
        _ => {
            return Ok(());
        }
    }

    Ok(())
}

pub fn event_thread(
    rx_register: Receiver<(usize, Sender<Event>)>,
    rx_event: Receiver<Event>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listeners = Arc::new(Mutex::new(HashMap::<usize, Sender<Event>>::new()));

    let listeners_register = listeners.clone();
    let handle = spawn(move || loop {
        let (id, ch) = rx_register.recv().expect("Event thread register listener");
        let mut listeners_guard = listeners_register.lock().unwrap();
        listeners_guard.insert(id, ch);
    });

    loop {
        let ev = rx_event.recv()?;
        // send event to its receivers
    }

    //let _ = handle.join();
    //Ok(())
}

pub fn handle_data(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let msg = read_message(&mut stream)?;
        let data = msg.as_slice();
        // send to parser and get command
    }
}

pub fn handle_event(
    mut stream: TcpStream,
    tx_register: Sender<(usize, Sender<Event>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    //configure event thread

    loop {
        let msg = read_message(&mut stream)?;
        // parse msg as EventCommand
        // until listen command received, execute commands
        break;
    }

    let (tx_event_listener, rx_event_listener) = channel();
    tx_register.send((0, tx_event_listener))?;
    loop {
        let msg = rx_event_listener.recv()?;
        // event
    }
}

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

pub fn read_message(stream: &mut TcpStream) -> Result<Message, Box<dyn std::error::Error>> {
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

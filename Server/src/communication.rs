use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

mod util;

use crate::database::Database;
use crate::*;
use util::read_message;

pub fn handle_client<T, E, P, C, D, CR>(
    mut stream: T,
    tx_register: Sender<(usize, Sender<E>)>,
    tx_event: Sender<E>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Read + Write,
    E: Event,
    P: Parser,
    C: Command,
    D: Database,
    CR: CommandResult,
{
    // wait for intent, data / event
    let mut intent = [0u8; 1];
    stream.read_exact(&mut intent)?;

    match intent[0] {
        0u8 => {
            // version 0, data, language = json
            handle_data::<T, E, P, C, D, CR>(stream, tx_event)?;
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

pub fn event_thread<V>(
    rx_register: Receiver<(usize, Sender<V>)>,
    rx_event: Receiver<V>,
) -> Result<(), Box<dyn std::error::Error>>
where
    V: Event + Send + 'static,
{
    let listeners = Arc::new(Mutex::new(HashMap::<usize, Sender<V>>::new()));

    let listeners_register = listeners.clone();
    spawn(move || loop {
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

pub fn handle_data<T, E, P, C, D, CR>(
    mut stream: T,
    tx_event: Sender<E>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Read + Write,
    E: Event,
    P: Parser,
    C: Command,
    D: Database,
    CR: CommandResult,
{
    loop {
        let msg = read_message(&mut stream)?;
        let data = msg.as_slice();
        let comm: C = P::parse_command(&data).unwrap();
        if comm.is_terminate() {
            return Ok(());
        }
        match D::run::<C, CR, E>(comm) {
            Ok((cr, evs)) => {
                // send result
                let data = P::serialize_command_result(cr).expect("Command result serialize error");
                if let Err(_) = stream.write(data.as_slice()) {
                    // assume socket closed
                    return Ok(());
                }
                let _ = evs.into_iter().map(|ev| tx_event.send(ev));
                // send events
            }
            Err(_) => {}
        }
    }
}

pub fn handle_event<T, V>(
    mut stream: T,
    tx_register: Sender<(usize, Sender<V>)>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Read,
    V: Event,
{
    //configure event thread

    loop {
        let msg = read_message(&mut stream)?;
        // parse msg as EventCommand
        // until listen command received, execute commands
        break;
    }

    let (tx_event_listener, rx_event_listener) = channel();
    tx_register
        .send((0, tx_event_listener))
        .unwrap_or_else(|_| unimplemented!());

    loop {
        let msg = rx_event_listener.recv()?;
        // event
    }
}

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

mod util;

use crate::database::Database;
use crate::*;
use util::read_message;

pub fn handle_client<T, E, P, C, D, CR, CO, EC>(
    mut stream: T,
    tx_register: Sender<(usize, Sender<CO>)>,
    tx_event: Sender<E>,
    id_counter: Arc<AtomicUsize>,
    db: Arc<RwLock<D>>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Read + Write,
    E: Event,
    P: Parser,
    C: Command,
    D: Database,
    CR: CommandResult,
    CO: EventContent,
    EC: EventCommand,
{
    // wait for intent, data / event
    let mut intent = [0u8; 1];
    stream.read_exact(&mut intent)?;

    match intent[0] {
        0u8 => {
            // version 0, data, language = json
            handle_data::<T, E, P, C, D, CR>(stream, tx_event, db)?;
        }
        1u8 => {
            // version 0, event, language = json
            handle_event::<T, P, D, CO, EC>(stream, tx_register, id_counter, db)?;
        }
        _ => {
            return Ok(());
        }
    }

    Ok(())
}

pub fn event_thread<V, CO>(
    rx_register: Receiver<(usize, Sender<CO>)>,
    rx_event: Receiver<V>,
) -> Result<(), Box<dyn std::error::Error>>
where
    V: Event<Content = CO>,
    CO: EventContent + Clone + Send + 'static,
{
    let listeners = Arc::new(Mutex::new(HashMap::<usize, Sender<CO>>::new()));

    let listeners_register = listeners.clone();
    spawn(move || loop {
        let (id, ch) = rx_register.recv().expect("Event thread register listener");
        let mut listeners_guard = listeners_register.lock().unwrap();
        listeners_guard.insert(id, ch);
    });

    loop {
        let ev = rx_event.recv()?;
        let targets = ev.get_target();
        if targets.len() > 0 {
            let listeners_guard = listeners.lock().unwrap();
            for t in targets {
                if let Some(listener) = listeners_guard.get(t) {
                    let content = ev.get_content().clone();
                    let _ = listener.send(content);
                }
            }
        }
        // send event to its receivers
    }

    //let _ = handle.join();
    //Ok(())
}

pub fn handle_data<T, E, P, C, D, CR>(
    mut stream: T,
    tx_event: Sender<E>,
    db: Arc<RwLock<D>>,
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
        match db.write().unwrap().run::<C, CR, E>(comm) {
            Ok((cr, evs)) => {
                // send result
                let data = P::serialize_command_result(cr).expect("Command result serialize error");
                if let Err(_) = stream.write(data.as_slice()) {
                    // assume socket closed
                    return Ok(());
                }
                let _ = evs.into_iter().map(|ev| tx_event.send(ev));
            }
            Err(_) => {}
        }
    }
}

pub fn handle_event<T, P, D, CO, EC>(
    mut stream: T,
    tx_register: Sender<(usize, Sender<CO>)>,
    id_counter: Arc<AtomicUsize>,
    db: Arc<RwLock<D>>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Read + Write,
    CO: EventContent,
    P: Parser,
    EC: EventCommand,
    D: Database,
{
    //configure event thread
    loop {
        let data = read_message(&mut stream)?;
        if let Ok(msg) = P::parse_ev_command::<EC>(data.as_slice()) {
            if msg.is_listen() {
                break;
            } else {
                db.write().unwrap().run_ev_command(msg);
            }
        } else {
            return Ok(()); //TODO: parse error
        }
    }

    let id = id_counter.fetch_add(1, Ordering::AcqRel);

    let (tx_event_listener, rx_event_listener) = channel();
    tx_register
        .send((id, tx_event_listener))
        .unwrap_or_else(|_| unimplemented!());

    loop {
        let msg = rx_event_listener.recv()?;
        let data = P::serialize_ev_content(msg).expect("serialize ev content error");
        if let Err(_) = stream.write(data.as_slice()) {
            return Ok(());
        }
    }
}

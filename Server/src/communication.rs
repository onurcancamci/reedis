use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

mod util;

use crate::common_traits::Database;
use crate::*;
use util::read_message;

pub fn handle_client<T, E, P, C, D, CR, CO, EC, TA>(
    stream: &mut T,
    tx_register: Sender<(usize, Sender<CO>)>,
    tx_event: Sender<E>,
    id_counter: Arc<AtomicUsize>,
    db: Arc<RwLock<D>>,
) -> Result<(), MyError>
where
    T: Read + Write,
    E: Event,
    P: Parser<ParsedCommand = D::Command, ParsedEvCommand = EC>,
    D: Database<CommandResult = CR, Event = E, Table = TA>,
    CR: CommandResult<Table = TA>,
    CO: EventContent,
    EC: EventCommand,
    TA: Table<Event = E>,
{
    // wait for intent, data / event
    let mut intent = [0u8; 1];
    stream
        .read_exact(&mut intent)
        .map_err(|_| MyError::SocketError)?;

    match intent[0] {
        0u8 => {
            drop(tx_register);
            // version 0, data, language = json
            handle_data::<T, E, P, D, CR, TA>(stream, tx_event, db)?;
        }
        1u8 => {
            drop(tx_event);
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
) -> Result<(), MyError>
where
    V: Event<Content = CO>,
    CO: EventContent + Clone + Send + 'static,
{
    let listeners = Arc::new(Mutex::new(HashMap::<usize, Sender<CO>>::new()));

    let listeners_register = listeners.clone();
    let register_handle = spawn(move || loop {
        let register_res = rx_register.recv();
        if register_res.is_err() {
            return MyError::EventChannelClosed;
        }

        let (id, ch) = register_res.unwrap();
        let mut listeners_guard = listeners_register.lock().unwrap();
        listeners_guard.insert(id, ch);
    });

    loop {
        let ev = rx_event.recv().map_err(|_| MyError::EventChannelClosed);
        match ev {
            Ok(ev) => {
                let target = ev.get_target();
                let listeners_guard = listeners.lock().unwrap();

                if let Some(listener) = listeners_guard.get(&target) {
                    let content = ev.get_content().clone();
                    let _ = listener.send(content);
                    // listener error might be temporary
                    // TODO: handle better
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    Err(register_handle.join().unwrap())
}

pub fn handle_data<T, E, P, D, CR, TA>(
    stream: &mut T,
    tx_event: Sender<E>,
    db: Arc<RwLock<D>>,
) -> Result<(), MyError>
where
    T: Read + Write,
    E: Event,
    P: Parser<ParsedCommand = D::Command>,
    D: Database<CommandResult = CR, Event = E, Table = TA>,
    CR: CommandResult<Table = TA>,
    TA: Table<Event = E>,
{
    loop {
        let msg = read_message(stream).map_err(|_| MyError::SocketError)?;
        let data = msg.as_slice();
        let comm: D::Command = P::parse_command(&data).unwrap();
        if comm.is_terminate() {
            return Ok(());
        }
        match db.write().unwrap().run(comm) {
            Ok((cr, evs)) => {
                // send result
                let data = P::serialize_command_result(cr).expect("Command result serialize error");
                if let Err(_) = stream.write(data.as_slice()) {
                    // assume socket closed
                    return Err(MyError::SocketError);
                }
                evs.into_iter().for_each(|ev| {
                    let _ = tx_event.send(ev);
                });
            }
            Err(_) => {
                // TODO: return error to client
            }
        }
    }
}

pub fn handle_event<T, P, D, CO, EC>(
    stream: &mut T,
    tx_register: Sender<(usize, Sender<CO>)>,
    id_counter: Arc<AtomicUsize>,
    db: Arc<RwLock<D>>,
) -> Result<(), MyError>
where
    T: Read + Write,
    CO: EventContent,
    P: Parser<ParsedEvCommand = EC>,
    EC: EventCommand,
    D: Database,
{
    //configure event thread
    loop {
        let data = read_message(stream).map_err(|_| MyError::SocketError)?;
        if let Ok(msg) = P::parse_ev_command(data.as_slice()) {
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
        .expect("Event thread is not there");

    drop(tx_register);

    loop {
        let msg = rx_event_listener
            .recv()
            .map_err(|_| MyError::EventChannelClosed)?;
        let data = P::serialize_ev_content(msg).expect("serialize ev content error");
        stream
            .write(data.as_slice())
            .map_err(|_| MyError::SocketError)?;
    }
}

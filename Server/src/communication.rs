use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

use crate::common_traits::Database;
use crate::*;

pub fn handle_client<T, E, P, D, EC>(
    stream: &mut T,
    tx_register: Sender<(usize, Sender<E::Content>)>,
    tx_event: Sender<E>,
    id_counter: Arc<AtomicUsize>,
    db: Arc<RwLock<D>>,
) -> Result<(), MyError>
where
    T: Read + Write,
    E: Event,
    P: Parser<D::Command, EC, D::Table>,
    D: Database<E>,
    EC: EventCommand,
{
    // wait for intent, data / event
    let intent = P::read_intent(stream)?;

    match intent {
        StreamIntent::Data => {
            drop(tx_register);
            handle_data::<T, E, P, D, EC>(stream, tx_event, db)?;
        }
        StreamIntent::Event => {
            drop(tx_event);
            handle_event::<T, P, D, EC, E>(stream, tx_register, id_counter, db)?;
        }
    }

    Ok(())
}

pub fn event_thread<V>(
    rx_register: Receiver<(usize, Sender<V::Content>)>,
    rx_event: Receiver<V>,
) -> Result<(), MyError>
where
    V: Event,
{
    let listeners = Arc::new(Mutex::new(HashMap::<usize, Sender<V::Content>>::new()));

    let listeners_register = Arc::clone(&listeners);
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

pub fn handle_data<T, E, P, D, EC>(
    stream: &mut T,
    tx_event: Sender<E>,
    db: Arc<RwLock<D>>,
) -> Result<(), MyError>
where
    T: Read + Write,
    E: Event,
    P: Parser<D::Command, EC, D::Table>,
    D: Database<E>,
    EC: EventCommand,
{
    loop {
        let comm: D::Command = P::read_command(stream)?;
        if comm.is_terminate() {
            return Ok(());
        }
        let q_result = if comm.is_mutator() {
            db.write().unwrap().run_mutable(comm)
        } else {
            db.read().unwrap().run(comm)
        };
        match q_result {
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
            Err(err) => {
                let cr = D::CommandResult::new_error_result(err);
                let data = P::serialize_command_result(cr).expect("Command result serialize error");
                if let Err(_) = stream.write(data.as_slice()) {
                    // assume socket closed
                    return Err(MyError::SocketError);
                }
            }
        }
    }
}

pub fn handle_event<T, P, D, EC, E>(
    stream: &mut T,
    tx_register: Sender<(usize, Sender<E::Content>)>,
    id_counter: Arc<AtomicUsize>,
    db: Arc<RwLock<D>>,
) -> Result<(), MyError>
where
    T: Read + Write,
    P: Parser<D::Command, EC, D::Table>,
    EC: EventCommand,
    D: Database<E>,
    E: Event,
{
    //configure event thread
    loop {
        let comm = P::read_ev_command(stream)?;
        if comm.is_listen() {
            break;
        } else {
            db.write().unwrap().run_ev_command(comm);
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

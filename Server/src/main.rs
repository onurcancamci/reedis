use communication::{event_thread, handle_client};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

mod communication;
mod database;
mod error;
mod message_types;
mod parser;
mod tests;

pub use error::*;
pub use message_types::*;
pub use parser::*;

fn main() -> std::io::Result<()> {
    // let (tx_register, rx_register) = channel::<(usize, Sender<MainEvent>)>(); //(id, Sender to event socket)
    // let (tx_event, rx_event) = channel::<MainEvent>();

    // spawn(|| {
    //     event_thread(rx_register, rx_event).expect("Event Thread Crash");
    // });
    // let listener = TcpListener::bind("127.0.0.1:7071")?;
    // for stream in listener.incoming() {
    //     let tx_register = tx_register.clone();
    //     let tx_event = tx_event.clone();
    //     if let Ok(stream) = stream {
    //         spawn(move || {
    //             let _ = handle_client::<TcpStream, MainEvent, TemporaryParser>(
    //                 stream,
    //                 tx_register,
    //                 tx_event,
    //             );
    //         });
    //     }
    // }
    Ok(())
}

// structure
//
// Comm ---[&[u8]]--> Parser
// Comm <--[Command]- Parser
// Comm may consume command to set connections internal state (event/data/disconnect...)
//
//
// Comm(Data):
// Comm ---[Command]----------> DB
// Comm <--[Result, Event[]]--- DB
// Comm ---[Result]-----------> Parser
// Comm <--[&[u8]]------------- Parser
// Comm ---[Event[]-----------> Events
//
// Comm(Event):
//
// Comm ---[Command]----------> Events
// Events -[Event]------------> Parser
// Events <[&[u8]]------------- Parser
// Comm <--[&[u8]]------------- Events
//
// Comm may send commands to Events module for configuring what events to receive etc
//
// info:
//
// 32 bit by default

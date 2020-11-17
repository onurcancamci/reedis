use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;

mod client;
mod message_types;

pub use message_types::*;

fn main() -> std::io::Result<()> {
    let (tx_register, rx_register) = channel::<(usize, Sender<Event>)>(); //(id, Sender to event socket)
    let (tx_event, rx_event) = channel::<Event>();

    spawn(|| {
        client::event_thread(rx_register, rx_event).expect("Event Thread Crash");
    });
    let listener = TcpListener::bind("127.0.0.1:7071")?;
    for stream in listener.incoming() {
        let tx_register = tx_register.clone();
        if let Ok(stream) = stream {
            spawn(move || {
                let _ = client::handle_client(stream, tx_register);
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn spin() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:7071")?;
        for stream in listener.incoming() {
            let mut stream = stream.expect("err");
            stream.set_nonblocking(true).unwrap();
            //stream
            //    .set_read_timeout(Some(std::time::Duration::from_millis(2)))
            //    .unwrap();

            loop {
                let mut buf = [0; 1024];
                let res = stream.read(&mut buf);
                match res {
                    Ok(len) => println!("READ {}", len),
                    Err(err) => match err.kind() {
                        std::io::ErrorKind::WouldBlock => {
                            //std::thread::sleep_ms(2);
                            std::thread::park();
                            //std::thread::yield_now();
                        }
                        _ => println!("ERROR"),
                    },
                }
            }
        }
        Ok(())
    }
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

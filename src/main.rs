#![feature(try_blocks)]
#![feature(box_syntax)]
#![allow(dead_code, unused_imports)]

mod app_result;
mod command_generator;
mod events;
mod executor;
mod serializer;
mod socket_provider;
mod types;
mod util;

pub use app_result::*;
//pub use async_trait::async_trait;
pub use command_generator::*;
pub use events::*;
pub use executor::*;
//pub use futures::future::{join, select};
pub use serializer::*;
pub use socket_provider::*;
use std::collections::VecDeque;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;
pub use types::*;
pub use util::*;

const STACK_SIZE: usize = 16 * 1024 * 1024;
const BUFFER_SIZE: usize = 1024 * 16;
const SIZE_USIZE: usize = std::mem::size_of::<usize>();
const MAX_NESTED_DEPTH: usize = 1024;

type MainTable = Arc<RwLock<Table>>;
type TPath = VecDeque<String>;

//#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id_ct = Arc::from(RwLock::new(0));
    let (ev_ch_sender, ev_ch_receiver) = channel();
    let (ev_int_sender, ev_int_receiver) = channel();

    /* let child_main = std::thread::Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(move || run(ev_ch_sender.clone(), ev_int_sender.clone(), id_ct.clone()))
    .unwrap(); */
    let t1 = spawn(move || run(ev_ch_sender.clone(), ev_int_sender.clone(), id_ct.clone()));

    /* let child_event = std::thread::Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(move || event_thread(ev_ch_receiver, ev_int_receiver))
    .unwrap(); */

    let t2 = spawn(move || event_thread(ev_ch_receiver, ev_int_receiver));

    // Wait for thread to join
    //let (_, _) = join(child_main.join().unwrap(), child_event.join().unwrap());

    let _ = t1.join();
    let _ = t2.join();
    Ok(())
}

fn event_thread(
    ev_ch_receiver: Receiver<(Sender<CommandInto>, i32)>,
    ev_int_receiver: Receiver<EventObj>,
) {
    let t1 = spawn(move || {
        loop {
            let received = ev_ch_receiver.recv();
            if let Ok(received) = received {
                // do stuff
                //println!("received evch: {:#?}", received);
            } else {
                //eprintln!("recv evch err");
                return;
            }
        }
    });
    let t2 = spawn(move || {
        loop {
            let received = ev_int_receiver.recv();
            if let Ok(received) = received {
                // do stuff
                //println!("received evint: {:#?}", received);
            } else {
                //eprintln!("recv evin err");
                return;
            }
        }
    });
    let _ = t1.join();
    let _ = t2.join();
}

fn run(
    ev_ch_sender: Sender<(Sender<CommandInto>, i32)>,
    ev_int_sender: Sender<EventObj>,
    id_ct: Arc<RwLock<i32>>,
) {
    let main_table = Arc::from(RwLock::from(Table::new(VecDeque::new())));
    let listener = TcpListener::bind("127.0.0.1:7071").unwrap();
    std::thread::Builder::new().spawn(send).unwrap();

    loop {
        let (socket, _) = listener.accept().unwrap();
        let main_table = main_table.clone();
        let event_socket = socket.try_clone().unwrap();
        let mut sp = SocketProvider::new(socket);

        let id = {
            let mut id_ct_ref = id_ct.write().unwrap();
            let id = *id_ct_ref;
            *id_ct_ref += 1;
            id
        };

        let (ev_sender, ev_receiver) = channel();
        ev_ch_sender.send((ev_sender, id)).unwrap();

        let ev_int_sender = ev_int_sender.clone();

        let t1 = spawn(move || {
            loop {
                let result: AppResult<()> = try {
                    let len = sp.read_packet()?;
                    let sp_content = sp.content()?;
                    let cw = CommandWrapper::new(&sp_content[0..len])?;
                    let (result, events) = Executor::execute(&cw, main_table.clone())?;
                    let result_serialized = Serializer::serialize(&result)?;
                    let _ = sp.write(result_serialized.as_slice())?;
                    for ev in events {
                        ev_int_sender
                            .send(ev)
                            .map_err(|_| AppError::ChannelWriteError)?;
                    }
                    //println!("{:?}", main_table.read().unwrap())
                };
                if let Err(e) = result {
                    eprintln!(
                        "{:#?} {}",
                        e,
                        main_table.clone().read().unwrap().byte_size()
                    );

                    std::process::exit(0);
                    return Err(AppError::SocketTerminated) as AppResult<()>;
                }
            }
        });
        let t2 = spawn(move || {
            loop {
                let received = ev_receiver.recv();
                if let Ok(received) = received {
                    // do stuff
                    println!("received: {:#?}", received);
                } else {
                    return;
                }
            }
        });
        let _ = t1.join();
        let _ = t2.join();
    }
}

fn send() {
    let _ = std::process::Command::new("bash")
        .arg("-c")
        .arg("cat /home/onurcan/Code/reedis/reedis.txt | nc localhost 7071")
        .stdout(std::process::Stdio::null())
        .spawn();
    //println!("{:?}", child.unwrap().stdout.unwrap());
}
mod test {
    use crate::*;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::prelude::*;
    use std::net::TcpStream;

    //#[tokio:test]
}

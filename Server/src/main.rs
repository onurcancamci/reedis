#![feature(try_blocks)]
#![feature(box_syntax)]
#![allow(dead_code, unused_imports)]

mod app_result;
mod array;
mod command_generator;
mod executor;
mod serializer;
mod socket_provider;
mod table;
mod types;
mod util;

pub use app_result::*;
pub use array::*;
pub use command_generator::*;
pub use executor::*;
pub use serializer::*;
pub use socket_provider::*;
use std::collections::VecDeque;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
pub use table::*;
pub use types::*;
pub use util::*;

const STACK_SIZE: usize = 16 * 1024 * 1024;
const BUFFER_SIZE: usize = 1024 * 16;
const SIZE_USIZE: usize = std::mem::size_of::<usize>();

type MainTable = Arc<Mutex<Table>>;
type TPath = VecDeque<String>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let child = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap().await?;
    Ok(())
}

fn send() {
    let _ = std::process::Command::new("bash")
        .arg("-c")
        .arg("cat /home/onurcan/Code/reedis/reedis.txt | nc localhost 7071")
        .stdout(std::process::Stdio::null())
        .spawn();
    //println!("{:?}", child.unwrap().stdout.unwrap());
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let main_table = Arc::from(Mutex::from(Table::new(VecDeque::new())));
    let listener = TcpListener::bind("127.0.0.1:7071")?;
    //std::thread::Builder::new().spawn(send).unwrap();

    loop {
        let (socket, _) = listener.accept()?;
        let main_table = main_table.clone();

        tokio::spawn(async move {
            let mut sp = SocketProvider::new(socket);
            loop {
                let result: AppResult<()> = try {
                    let len = sp.read_packet()?;
                    let cw = CommandWrapper::new(&sp.content()?[0..len])?;
                    let result = Executor::execute(&cw, main_table.clone())?;
                    /* println!(
                        "result = {:#?}\n",
                        result,
                        //Serializer::serialize(&result)?.as_slice()
                    ); */
                    let _ = sp.write(Serializer::serialize(&result)?.as_slice())?;
                    println!("{:#?}", main_table.lock().unwrap())
                };
                if let Err(e) = result {
                    eprintln!(
                        "{:#?} {}",
                        e,
                        main_table.clone().lock().unwrap().byte_size()
                    );

                    //std::process::exit(0);
                    //return;
                }
            }
        });
    }
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

#![feature(try_blocks)]
#![feature(box_syntax)]
#![allow(dead_code, unused_imports)]

mod app_result;
mod executor;
mod serializer;
mod socket_provider;
mod table;
mod types;
mod util;

pub use app_result::*;
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
        .arg("cat /home/onurcan/Code/Rust/reedis2/reedis.txt | nc localhost 7071")
        //.stdout(std::process::Stdio::inherit())
        .spawn();
    //println!("{:?}", child.unwrap().stdout.unwrap());
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let main_table = Arc::from(Mutex::from(Table::new()));
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
                    /* let cw2 = CommandInto::new_raw(
                        cw.command_type().clone(),
                        cw.raw_args()
                            .iter()
                            .map(|dw| DataInto::new_raw(dw.data_type().clone(), dw.data().unwrap()))
                            .collect(),
                    ); */
                    /* let mut b1 = [0u8; 2048];
                    let mut b2 = [0u8; 2048];
                    cw.read_into(&mut b1[0..cw.size()])?;
                    cw2.read_into(&mut b2[0..cw2.size()])?; */
                    /* for (i, b) in b1.iter().enumerate() {
                        println!("{} {}", *b, b2[i]);
                        if *b != b2[i] {
                            println!("NOT EQ");
                        }
                    } */
                    /* cw.raw_args().iter().for_each(|a| {
                        a.size();
                    });
                    println!(
                        "ALL {:?} {:?} \n\n{:#?}\n\n{:#?}",
                        cw.size(),
                        cw2.size(),
                        cw,
                        cw2
                    ); */
                    let result = Executor::execute(&cw, main_table.clone())?;
                    let _ = sp.write(Serializer::serialize(&result)?.as_slice())?;
                    //println!("write len {}", write_len);
                    //println!("{:#?}", main_table.lock().unwrap())
                };
                if let Err(e) = result {
                    eprintln!(
                        "{:#?} {}",
                        e,
                        main_table.clone().lock().unwrap().byte_size()
                    );

                    std::process::exit(0);
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

    #[tokio::test]
    async fn write_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect("127.0.0.1:7070")?;

        //set
        stream.write(&28usize.to_le_bytes())?; //total size
        stream.write(&2u16.to_le_bytes())?; //command
        stream.write(&5usize.to_le_bytes())?; //arg1 size
        stream.write(&1u16.to_le_bytes())?; //key type path/string
        stream.write(&"xyz".as_bytes())?; //key content
        stream.write(&5usize.to_le_bytes())?; //arg2 size
        stream.write(&1u16.to_le_bytes())?; //key type path/string
        stream.write(&"xyz".as_bytes())?; //key content

        //tokio::spawn(async move { loop {} }).await?;
        Ok(())
    }

    #[tokio::test]
    async fn write_test_file() -> Result<(), Box<dyn std::error::Error>> {
        let val_proto = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let mut out = vec![];
        let mut total_size = 0usize;
        for k in 0..1_000_000 {
            let path = k.to_string();
            let path_len = path.as_bytes().len() + 2;
            let val_len = val_proto.as_bytes().len() + 2;
            let total_len = 16 + val_len + path_len + 2;
            total_size += path.as_bytes().len() + 8 + val_len + 8;
            out.extend_from_slice(&(total_len as usize).to_le_bytes()); //total size
            out.extend_from_slice(&2u16.to_le_bytes()); //command
            out.extend_from_slice(&(path_len as usize).to_le_bytes()); //arg1 size
            out.extend_from_slice(&1u16.to_le_bytes()); //key type path/string
            out.extend_from_slice(&path.as_bytes()); //key content
            out.extend_from_slice(&(val_len as usize).to_le_bytes()); //arg2 size
            out.extend_from_slice(&1u16.to_le_bytes()); //key type path/string
            out.extend_from_slice(&val_proto.as_bytes()); //key content
        }
        out.extend_from_slice(&2usize.to_le_bytes()); //total size
        out.extend_from_slice(&0xffffu16.to_le_bytes()); //command
        println!("{}", total_size);
        //let mut file = File::create("reedis2.txt")?;
        //file.write_all(out.as_slice())?;

        Ok(())
    }
}

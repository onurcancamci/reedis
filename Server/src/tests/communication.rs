mod mock;

use crate::communication::{event_thread, handle_client};
use crate::error::MyError;
use mock::*;
use std::io::{Read, Write};
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

#[test]
fn simple_flow() {
    let (tx_register, rx_register) = channel::<(usize, Sender<MockEventContent>)>();
    let (tx_event, rx_event) = channel::<MockEvent>();
    let id_counter = Arc::new(AtomicUsize::new(0));
    let db = Arc::new(RwLock::new(MockDatabase {}));

    let context = Arc::new(RwLock::new(MockExecutionContext {}));
    let context_ev = Arc::clone(&context);
    let context_client = Arc::clone(&context);

    let ev_handle = spawn(|| event_thread(rx_register, rx_event, context_ev));

    // Event listener client

    let ev_client_tcp = MockTcpStream::new(vec![1, 1], 10); // ev channel, 1 listen
    let ev_client_tcp_mut = Arc::new(Mutex::new(ev_client_tcp));
    let c_ev_client_tcp_mut = ev_client_tcp_mut.clone();

    let c_tx_register = tx_register.clone();
    let c_tx_event = tx_event.clone();
    let c_id_counter = id_counter.clone();
    let c_db = db.clone();
    let ev_listen_handle = spawn(move || {
        handle_client::<MockTcpStream, MockEvent, MockParser, MockDatabase, MockExecutionContext>(
            &mut *c_ev_client_tcp_mut.lock().unwrap(),
            c_tx_register,
            c_tx_event,
            c_id_counter,
            c_db,
            Arc::clone(&context),
        )
    });

    std::thread::sleep(std::time::Duration::from_millis(200));

    let client_tcp = MockTcpStream::new(vec![0, 0, 1], 10); // data channel, 1 non-terminate, 1 (terminate)
    let client_tcp_mut = Arc::new(Mutex::new(client_tcp));
    let c_client_tcp_mut = client_tcp_mut.clone();

    let c_tx_register = tx_register.clone();
    let c_tx_event = tx_event.clone();
    let c_id_counter = id_counter.clone();
    let c_db = db.clone();
    let data_handle = spawn(move || {
        handle_client::<MockTcpStream, MockEvent, MockParser, MockDatabase, MockExecutionContext>(
            &mut *c_client_tcp_mut.lock().unwrap(),
            c_tx_register,
            c_tx_event,
            c_id_counter,
            c_db,
            context_client,
        )
    });

    assert!(data_handle.join().unwrap().is_ok());

    drop(tx_event);
    drop(tx_register);
    assert_eq!(
        ev_handle.join().unwrap().unwrap_err(),
        MyError::EventChannelClosed
    );

    assert_eq!(
        ev_listen_handle.join().unwrap().unwrap_err(),
        MyError::EventChannelClosed
    );

    //TODO: fix this
    //event thread not receiving event
    //before, commands on tables returned result and events vector
    //now, tables send events directly to event thread via ExecutionContext
    assert_eq!(ev_client_tcp_mut.lock().unwrap().out, vec![32]);
    assert_eq!(client_tcp_mut.lock().unwrap().out, vec![33]);
}

#[test]
fn mock_tcp_read_exact() {
    let inp = vec![0u8, 1u8, 2u8];
    let mut mtcp = MockTcpStream::new(inp, 3);

    let mut buf = [0u8; 2];

    let res = mtcp.read_exact(&mut buf);

    assert!(res.is_ok());
    assert_eq!(buf, [0u8, 1u8]);
}

#[test]
fn mock_tcp_read_exact_cant_fill_buffer() {
    let inp = vec![];
    let mut mtcp = MockTcpStream::new(inp, 3);

    let mut buf = [0u8; 2];

    let res = mtcp.read_exact(&mut buf);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), std::io::ErrorKind::UnexpectedEof);
}

#[test]
fn mock_tcp_write() {
    let inp = vec![0u8, 1u8, 2u8];
    let mut mtcp = MockTcpStream::new(vec![], 3);

    let res = mtcp.write(&inp);

    assert!(res.is_ok());
    assert_eq!(mtcp.out.as_slice(), [0u8, 1u8, 2u8]);
}

#[test]
fn mock_tcp_write_broken_pipe() {
    let inp = vec![0u8, 1u8, 2u8];
    let mut mtcp = MockTcpStream::new(vec![], 0);

    let res = mtcp.write(&inp);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), std::io::ErrorKind::BrokenPipe);
}

use std::thread;
use std::net::SocketAddr;

use tokio::prelude::*;
use futures::sync::mpsc;

pub mod tcp {
    use bytes::BytesMut;

    use std::io;
    use std::net::SocketAddr;

    use tokio::prelude::*;

    pub fn connect(addr: &SocketAddr, stdin: Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>)
                   -> Box<Stream<Item=BytesMut, Error=io::Error> + Send> {}
}

pub fn demo() {
    let addr = "127.0.0.1:8086".parse::<SocketAddr>().unwrap();
    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    thread::spawn(|| read_stdin(stdin_tx));
    let stdin_rx = stdin_rx.map_err(|_| panic!());

    tcp::connect(&addr, Box::new(stdin_rx))
}

fn read_stdin(mut tx: mpsc::Sender<Vec<u8>>) {}
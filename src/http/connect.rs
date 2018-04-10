use std::io;
use std::thread;
use std::net::SocketAddr;

use tokio;
use tokio::prelude::*;
use futures::sync::mpsc;
use futures::Stream;

pub mod codec {
    use std::io;

    use bytes::{BufMut, BytesMut};
    use tokio_io::codec::{Encoder, Decoder};

    pub struct Bytes;

    impl Decoder for Bytes {
        type Item = BytesMut;
        type Error = io::Error;

        fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
            if buf.len() > 0 {
                let len = buf.len();
                Ok(Some(buf.split_to(len)))
            } else {
                Ok(None)
            }
        }
    }

    impl Encoder for Bytes {
        type Item = Vec<u8>;
        type Error = io::Error;

        fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
            buf.put(&data[..]);
            Ok(())
        }
    }
}

mod tcp {
    use bytes::BytesMut;

    use std::io;
    use std::net::SocketAddr;

    use tokio;
    use tokio::prelude::*;
    use tokio::net::TcpStream;

    use http::connect::codec::Bytes;

    pub fn connect(addr: &SocketAddr, stdin: Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>)
                   -> Box<Stream<Item=BytesMut, Error=io::Error> + Send> {
        let tcp = TcpStream::connect(addr);

        Box::new(tcp.map(move |stream| {
            let (sink, stream) = stream.framed(Bytes).split();

            tokio::spawn(stdin.forward(sink).then(|result| {
                if let Err(e) = result {
                    panic!("failed to write to socket: {}", e)
                }
                Ok(())
            }));
            stream
        }).flatten_stream())
    }
}

pub fn demo() {
    let addr = "10.16.76.197:8080".parse::<SocketAddr>().unwrap();
    let (stdin_tx, stdin_rx) = mpsc::channel(0);
    thread::spawn(|| read_stdin(stdin_tx));
    let stdin_rx = stdin_rx.map_err(|_| panic!());

    let stdout = tcp::connect(&addr, Box::new(stdin_rx));

    let mut out = io::stdout();
    tokio::run({
        stdout.for_each(move |chunk| {
            out.write_all(&chunk)
        })
            .map_err(|e| println!("error reading stdout; error = {:?}", e))
    });
}

fn read_stdin(mut tx: mpsc::Sender<Vec<u8>>) {
    let mut stdin = io::stdin();

    loop {
        let mut buf = vec![0; 1204];
        let n = match stdin.read(&mut buf) {
            Err(_) |
            Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx = match tx.send(buf).wait() {
            Ok(tx) => tx,
            Err(_) => break,
        };
    }
}
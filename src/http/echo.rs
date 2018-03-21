use futures::Stream;
use futures::Future;
use tokio;
use tokio::io;
use tokio::io::AsyncRead;
use tokio::net::TcpListener;

pub fn demo() {
    let addr = "127.0.0.1:8086".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let done = listener.incoming()
        .map_err(|e| println!("failed to accept socket; error ={:?}", e))
        .for_each(move |socket| {
            let (reader, writer) = socket.split();
            let amt = io::copy(reader, writer);
            let msg = amt.then(move |result| {
                match result {
                    Ok((amt, _, _)) => println!("wrote {} bytes", amt),
                    Err(e) => println!("error: {}", e),
                }
                Ok(())
            });

            tokio::spawn(msg)
        });

    tokio::run(done);
}
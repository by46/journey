use tokio;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn demo() {
    let addr = "127.0.0.1:8086".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    let server = listener.incoming().for_each(|socket| {
        println!("accept socket; addr={:?}", socket.peer_addr().unwrap());
        let connection = io::write_all(socket, "hello world\n")
            .then(|response| {
                println!("wrote message; success={:?}", response.is_ok());
                Ok(())
            });
        tokio::spawn(connection);
        Ok(())
    }).map_err(|err| {
        println!("accept error : {:?}", err)
    });

    println!("server running on localhost:8086");
    tokio::run(server);
}
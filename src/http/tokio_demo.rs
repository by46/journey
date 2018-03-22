use futures::Stream;
use futures::Future;
use tokio;
use tokio::net::TcpListener;
use tokio_io::io;

pub fn demo() {
    let addr = "127.0.0.1:8086".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accept socket; address={:?}", socket.peer_addr().unwrap());
        let connection = io::write_all(socket, "hello world\n")
            .then(|res| {
                println!("wrote message; success={:?}", res.is_ok())
            });
        tokio::spawn(connection);
        Ok(())
    })
        .map_err(|err| {
            println!("accept error = {:?}", err)
        });
    tokio::run(server)
}
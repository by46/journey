//use futures::Future;
//use futures::future::lazy;
//use hyper::Body;
//use hyper::Response;
//use hyper::server::Http;
//use hyper::server::const_service;
//use hyper::server::service_fn;
//use pretty_env_logger;
//use tokio;
//
//static PHASE: &'static [u8] = b"hello world";
//
//pub fn demo() {
//    pretty_env_logger::init();
//    let addr = ((127, 0, 0, 1), 8080).into();
//    let new_service = const_service(service_fn(|_| {
//        let &mut response = Response::new();
//        response.set_body(Body::from(PHASE));
//        Ok(response)
//    }));
//    tokio::run
//}
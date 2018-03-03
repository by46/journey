use iron::Handler;
use iron::Iron;
use iron::Request;
use iron::Response;
use iron::IronError;
use iron::AroundMiddleware;
use iron::IronResult;
use iron::status;
use time;

enum LoggerMode {
    Silent,
    Tiny,
    Large,
}

struct Logger {
    mode: LoggerMode
}

struct LoggerHandler<H: Handler> { logger: Logger, handler: H }

impl Logger {
    fn new(mode: LoggerMode) -> Logger { Logger { mode } }
    fn log(&self, req: &Request, res: Result<&Response, &IronError>, time: u64) {
        match self.mode {
            LoggerMode::Silent => {}
            LoggerMode::Tiny => println!("Req: {:?}\n Res: {:?}\nTook:{}", req, res, time),
            LoggerMode::Large => println!("Request: {:?}\nResponse: {:?}\nResponse-Time: {}", req, res, time)
        }
    }
}

impl<H: Handler> Handler for LoggerHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let entry = time::precise_time_ns();
        let res = self.handler.handle(req);
        self.logger.log(req, res.as_ref(), time::precise_time_ns() - entry);
        res
    }
}

impl AroundMiddleware for Logger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(LoggerHandler {
            logger: self, handler,
        }) as Box<Handler>
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "hello world")))
}

pub fn demo() {
    let silent = Iron::new(Logger::new(LoggerMode::Silent)
        .around(Box::new(hello_world)));
    let tiny = Iron::new(Logger::new(LoggerMode::Tiny)
        .around(Box::new(hello_world)));
    let large = Iron::new(Logger::new(LoggerMode::Large)
        .around(Box::new(hello_world)));

    let _silent_listening = silent.http("0.0.0.0:4001").unwrap();
    let _tiny_listening = tiny.http("0.0.0.0:4002").unwrap();
    let _large_listening = large.http("0.0.0.0:4003").unwrap();
    println!("Servers Listening on 4001,4002,4003")
}
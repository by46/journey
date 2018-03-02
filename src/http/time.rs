use iron;
use iron::prelude::Chain;
use iron::prelude::Iron;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::prelude::Response;
use iron::BeforeMiddleware;
use iron::AfterMiddleware;
use iron::typemap;
use time::precise_time_ns;

use example::common;

struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello world")))
}

pub fn demo() {
    common::line();
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
    println!("On Listening");
}
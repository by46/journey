use time::precise_time_ns;
use iron;
use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::AfterMiddleware;
use iron::typemap;


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

fn hello(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "hello world")))
}

pub fn demo() {
    let mut chain = Chain::new(hello);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}
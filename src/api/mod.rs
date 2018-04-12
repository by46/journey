use time::precise_time_ns;
use iron;
use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::AfterMiddleware;
use iron::Handler;
use iron::typemap;
use iron::method::Method;
use std::str::FromStr;
use std::convert::AsRef;
use iron::status::Accepted;
use iron::mime::Mime;
use iron::modifiers::Redirect;
use iron::modifiers::RedirectRaw;
use iron::headers::AccessControlAllowOrigin;
use iron::modifiers::Header;

struct ResponseTime;

struct App {}

impl App {
    fn new() -> Self {
        App {}
    }

    fn options(&self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "")))
    }

    fn get(&self, request: &mut Request) -> IronResult<Response> {
        Ok(match request.url.path().join("/").as_ref() {
            "faq.htm" => Response::with((iron::status::Ok, "<!Newegg>")),
            "version" => Response::with((iron::status::Ok, "0.0.1")),
            "time" => {
                let mut response = Response::new();
                response.set_mut(Accepted);
                response.set_mut(mime!(Application/Css));
                response.set_mut(String::from("hello world"));
                response.set_mut(Redirect("http://localhost:3000/version".parse().unwrap()));
                response.set_mut(RedirectRaw(String::from("http://localhost:3000/faq.html")));
                response.set_mut(Header(AccessControlAllowOrigin::Any));

                response
            }
            _ => Response::with((iron::status::Ok, request.url.to_string())),
        })
    }

    fn post(&self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "post file")))
    }
    fn put(&self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "put method")))
    }
    fn delete(&self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "delete method")))
    }
}

impl Handler for App {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        match request.method {
            Method::Options => self.options(request),
            Method::Get => self.get(request),
            Method::Post => self.post(request),
            Method::Put => self.put(request),
            Method::Delete => self.delete(request),
            _ => Ok(Response::with((iron::status::MethodNotAllowed, "")))
        }
    }
}

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
    let app = App::new();
    Iron::new(app).http("localhost:3000").unwrap();
}
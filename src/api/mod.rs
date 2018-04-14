use time::precise_time_ns;
use iron;
use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::AfterMiddleware;
use iron::Handler;
use iron::typemap;
use iron::TypeMap;
use iron::method::Method;
use std::str::FromStr;
use std::convert::AsRef;
use iron::status::Accepted;
use iron::status::Found;
use iron::mime::Mime;
use iron::mime::TopLevel;
use iron::mime::SubLevel;
use iron::modifiers::Redirect;
use iron::modifiers::RedirectRaw;
use iron::headers;
use iron::headers::qitem;
use iron::headers::AccessControlAllowOrigin;
use iron::modifiers::Header;
use std::path::Path;

struct ResponseTime;

struct App {}
header! {
    (XCabinetName, "X-Cabinet-Name") => [String]
}

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
                println!("{:?}", request);
                let mut response = Response::new();
                response.set_mut(Accepted);
                response.set_mut(mime!(Application/Css));
                response.set_mut(Header(headers::Cookie(vec![String::from("hello")])));
                // accept
                let value = headers::Accept(vec![
                    qitem(Mime(TopLevel::Image, SubLevel::Html, vec![]))
                ]);
                response.set_mut(Header(value));

                // host
                response.set_mut(Header(headers::Host {
                    hostname: "www.newegg.com".to_owned(),
                    port: Some(80)
                }));
                response.set_mut(String::from("hello world"));

                // server
                response.set_mut(Header(headers::Server("engine/0.1.23".to_owned())));

                // X-Cabinet-Name
                response.set_mut(Header(XCabinetName("benjamin".to_owned())));
                response.set_mut(Redirect("http://localhost:3000/version".parse().unwrap()));
                response.set_mut(RedirectRaw(String::from("http://localhost:3000/faq.html")));
                response.set_mut(Header(AccessControlAllowOrigin::Any));

                response
            }
            "types" => {
                let mut response = Response::new();
                let name = String::from("population.json");
                let path = Path::new(&name);
                response.set_mut(Found);
                response.set_mut(path);
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
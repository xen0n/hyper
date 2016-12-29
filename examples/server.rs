//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Server, Service, Request, Response};


static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Response, hyper::Error>;

    fn call(&mut self, req: Request) -> Self::Future {
        ::futures::finished(match (req.method(), req.path()) {
            (&Get, Some("/")) | (&Get, Some("/echo")) => {
                Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)
            },
            (&Post, Some("/echo")) => {
                let mut res = Response::new();
                if let Some(len) = req.headers().get::<ContentLength>() {
                    res.headers_mut().set(len.clone());
                }
                res.with_body(req.body())
            },
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
        })
    }

}


fn main() {
    pretty_env_logger::init().unwrap();
    let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    let (listening, server) = server.standalone(|| Ok(Echo)).unwrap();
    println!("Listening on http://{}", listening);
    server.run();
}

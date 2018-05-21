extern crate futures;
extern crate hyper;

// 3rd-party imports

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

// service

struct Middleware;

const PHRASE: &'static str = "Hello, World!";

impl Service for Middleware {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE),
        ))
    }
}

// app

fn main() {
    let addr = "0.0.0.0:7777".parse().unwrap();

    let server = Http::new()
        .bind(&addr, || {
            let middleware = Middleware;
            Ok(middleware)
        })
        .unwrap();

    // TODO: cpu pooling
    println!(
        "Listening on http://{} with 1 thread.",
        server.local_addr().unwrap()
    );

    server.run().unwrap();
}

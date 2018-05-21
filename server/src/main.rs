extern crate hyper;

// 3rd-party imports

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

// service

const PHRASE: &'static str = "Hello, World!";

// app

fn main() {

    let server_address = "0.0.0.0:7777".parse().unwrap();

    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let new_service = || {
        // This is the `Service` that will handle the connection.
        // `service_fn_ok` is a helper to convert a function, that
        // returns a Response, into a `Service`.
        service_fn_ok(|_| {
            Response::new(Body::from(PHRASE))
        })
    };

    let server = Server::bind(&server_address)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", server_address);

    rt::run(server);
}

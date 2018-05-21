extern crate hyper;

// 3rd-party imports

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server, StatusCode};

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
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(PHRASE))
                .unwrap()
        })
    };

    let server = Server::bind(&server_address)
        .serve(new_service)
        .map_err(|e| {
            // TODO: error logging to a service
            eprintln!("server error: {}", e)
        });

    println!("Listening on http://{}", server_address);

    // Use tokio runtime to handle the thread-pooling: https://docs.rs/tokio/0.1.6/tokio/runtime/index.html
    rt::run(server);
}

extern crate futures;
extern crate hyper;

use hyper::Body;
use hyper::Client;
use hyper::Request;
use hyper::Response;
use hyper::rt::Future;
use hyper::Server;
use hyper::service::service_fn;

type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn echo(req: Request<Body>) -> BoxFut {
    let client = Client::new();
    let response_future = client.request(req)
        .and_then(|real_response| {
            println!("real_response: {:?}", real_response);
            Ok(real_response)
        });
    Box::new(response_future)
}

fn main() {
    let addr = ([127, 0, 0, 1], 1339).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running proxy on :1339");
    hyper::rt::run(server);
}

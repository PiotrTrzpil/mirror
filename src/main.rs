extern crate iron;
extern crate time;

use iron::prelude::*;

fn handler(req: &mut Request) -> IronResult<Response> {
    println!("Req: {:?}, {:?}", req, req.headers);
    Ok(Response::with((iron::status::Ok, format!("{:?}, {:?}", req, req.headers))))
}

fn main() {
    let chain = Chain::new(handler);
    let endpoint = "0.0.0.0:8333";
    println!("Starting mirror server on: {}", endpoint);
    Iron::new(chain).http(endpoint).unwrap();
}

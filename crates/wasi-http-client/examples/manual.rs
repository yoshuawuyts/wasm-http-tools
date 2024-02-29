use wasi::http::outgoing_handler::{handle, OutgoingRequest};
use wasi::http::types::{Fields, Method, Scheme};
use wasi::io::poll;

fn main() {
    // Construct an HTTP request
    let fields = Fields::new();
    let req = OutgoingRequest::new(fields);
    req.set_method(&Method::Get).unwrap();
    req.set_scheme(Some(&Scheme::Https)).unwrap();
    req.set_path_with_query(Some("/")).unwrap();
    req.set_authority(Some("example.com")).unwrap();

    // Send the request and wait for it to complete
    let res = handle(req, None).unwrap();
    let pollable = res.subscribe();
    poll::poll(&[&pollable]);

    // Request was successfully sent; we can now access the response and headers
    let res = res.get().unwrap().unwrap().unwrap();
    let headers = res.headers().entries();
    for (key, _value) in headers {
        println!("header: {key}");
    }
}

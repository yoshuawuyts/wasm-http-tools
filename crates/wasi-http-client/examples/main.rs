use wasi::http::{
    outgoing_handler::{handle, OutgoingRequest},
    types::{Fields, Method, Scheme},
};
use wasi_http_client::runtime::Poller;

fn main() {
    // Construct the runtime
    let mut poller = Poller::new();

    // Construct the request
    let fields = Fields::new();
    let req = OutgoingRequest::new(fields);
    req.set_method(&Method::Get).unwrap();
    req.set_scheme(Some(&Scheme::Https)).unwrap();
    req.set_path_with_query(Some("/")).unwrap();
    req.set_authority(Some("example.com")).unwrap();

    // Send the request and wait for it to complete
    let res = handle(req, None).unwrap();
    let pollable = res.subscribe();
    let key = poller.insert(pollable);
    poller.block_until();
    poller.remove(key);

    // Parse the response headers to find the length of the body
    let res = res.get().unwrap().unwrap().unwrap();
    let headers = res.headers().entries();
    let (_, content_length) = headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == "content-length")
        .expect("no content-length found; violates HTTP/1.1");
    let content_length = String::from_utf8(content_length.clone())
        .unwrap()
        .parse::<u64>()
        .unwrap();

    // Receive the request body
    // TODO: read smaller chunks than `content_length` at the same time
    let body = res.consume().unwrap();
    let body = body.stream().unwrap();
    let pollable = body.subscribe();
    let key = poller.insert(pollable);
    poller.block_until();
    poller.remove(key);

    // Parse the request body into a string and print it
    let bytes = body.read(content_length).unwrap();
    dbg!(String::from_utf8(bytes).unwrap());
}

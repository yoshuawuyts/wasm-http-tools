use wasi::http::{
    outgoing_handler::{handle, OutgoingRequest},
    types::{Fields, Method, RequestOptions, Scheme},
};
use wasi_http_client::Poller;

fn main() {
    let mut poller = Poller::new();
    let fields = Fields::new();
    let req = OutgoingRequest::new(fields);
    req.set_method(&Method::Get).unwrap();
    req.set_scheme(Some(&Scheme::Https)).unwrap();
    req.set_path_with_query(Some("/")).unwrap();
    req.set_authority(Some("example.com")).unwrap();
    let res = handle(req, None).unwrap();
    let pollable = res.subscribe();
    poller.insert(pollable);
    // assert!(&pollable.ready(), "pollable was not ready");
    poller.block_until();
    drop(poller);
    dbg!(res);
}

//! WASI-based HTTP client library
//!
//! # Examples
//!
//! ```text
//! // tbi
//! ```

#![forbid(rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use url::Url;

use wasi_async_runtime::Reactor;

mod method;
mod request;
mod response;

/// An HTTP client.
#[derive(Debug)]
pub struct Client {
    reactor: Reactor,
}

impl Client {
    /// Create a new instance of `Client`
    pub fn new(reactor: Reactor) -> Self {
        Self { reactor }
    }

    /// Send an HTTP request.
    pub async fn send(&self, req: Request) -> Response {
        let wasi_req = req.into();
        let res = wasi::http::outgoing_handler::handle(wasi_req, None).unwrap();
        self.reactor.wait_for(res.subscribe()).await;

        // TODO: handle errors without panicking
        let res = res.get().unwrap().unwrap().unwrap();
        Response::from_incoming(res, self.reactor.clone())
    }
}

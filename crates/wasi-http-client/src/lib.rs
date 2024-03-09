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

/// The `wasi-http-client` error type.
pub type Error = wasi::http::types::ErrorCode;

/// The `wasi-http-client` result type.
pub type Result<T> = std::result::Result<T, Error>;

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
    pub async fn send(&self, req: Request) -> Result<Response> {
        let wasi_req = req.into();
        let res = wasi::http::outgoing_handler::handle(wasi_req, None).unwrap();
        self.reactor.wait_for(res.subscribe()).await;

        // NOTE: the first `unwrap` is to ensure readiness, the second `unwrap`
        // is to trap if we try and get the response more than once. The final
        // `?` is go raise the actual error if there is one.
        let res = res.get().unwrap().unwrap()?;
        Ok(Response::from_incoming(res, self.reactor.clone()))
    }
}

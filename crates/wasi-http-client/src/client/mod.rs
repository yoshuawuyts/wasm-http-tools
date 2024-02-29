use crate::runtime;

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use url::Url;

mod method;
mod request;
mod response;

/// An HTTP client.
#[derive(Debug)]
pub struct Client {
    reactor: runtime::Reactor,
}

impl Client {
    /// Create a new instance of `Client`
    pub fn new(reactor: runtime::Reactor) -> Self {
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

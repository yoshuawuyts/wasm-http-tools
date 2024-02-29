use crate::runtime::Reactor;
use wasi::{
    http::types::{IncomingBody, IncomingResponse},
    io::streams::{InputStream, StreamError},
};

/// Stream 2kb chunks at a time
const CHUNK_SIZE: u64 = 2048;

/// An HTTP response
#[derive(Debug)]
pub struct Response {
    bytes_read: u64,
    content_length: u64,
    headers: Vec<(String, Vec<u8>)>,
    reactor: Reactor,

    // IMPORTANT: the order of these fields here matters. `incoming_body` must
    // be dropped before `body_stream`.
    body_stream: InputStream,
    _incoming_body: IncomingBody,
}

impl Response {
    pub(crate) fn from_incoming(incoming: IncomingResponse, reactor: Reactor) -> Self {
        let headers = incoming.headers().entries();

        let (_, content_length) = headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == "content-length")
            .expect("no content-length found; violates HTTP/1.1");
        let content_length = String::from_utf8(content_length.clone())
            .unwrap()
            .parse::<u64>()
            .unwrap();

        // `body_stream` is a child of `incoming_body` which means we cannot
        // drop the parent before we drop the child
        let incoming_body = incoming
            .consume()
            .expect("cannot call `consume` twice on incoming response");
        let body_stream = incoming_body
            .stream()
            .expect("cannot call `stream` twice on an incoming body");

        Self {
            bytes_read: 0,
            headers,
            content_length,
            body_stream,
            _incoming_body: incoming_body,
            reactor,
        }
    }

    // pub async fn read_to_end(self) -> Result<Vec<u8>, StreamError> {
    //     // Wait for an event to be ready
    //     self.reactor.wait_for(self.body_stream.subscribe()).await;

    //     // Read the bytes from the body stream
    //     let buf = self.body_stream.read(self.content_length);
    //     buf
    // }

    /// Get the next chunk from the HTTP body stream.
    pub async fn next_chunk(&mut self) -> Option<Result<Vec<u8>, StreamError>> {
        // Calculate how many bytes we can read
        let remaining = self.content_length - self.bytes_read;
        let len = remaining.min(CHUNK_SIZE);
        if len == 0 {
            return None;
        }

        // Wait for an event to be ready
        let pollable = self.body_stream.subscribe();
        self.reactor.wait_for(pollable).await;

        // Read the bytes from the body stream
        let buf = self.body_stream.read(len);
        self.bytes_read += len;
        Some(buf)
    }

    /// Get the HTTP headers from the impl
    pub fn headers(&self) -> &[(String, Vec<u8>)] {
        &self.headers
    }
}

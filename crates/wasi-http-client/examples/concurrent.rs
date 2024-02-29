use futures_concurrency::prelude::*;
use wasi_http_client::runtime::block_on;
use wasi_http_client::{Client, Method, Request, Response};

fn main() {
    block_on(|reactor| async {
        let client = Client::new(reactor);

        let url = "https://example.com".parse().unwrap();
        let req1 = Request::new(Method::Get, url);

        let url = "https://example.com".parse().unwrap();
        let req2 = Request::new(Method::Get, url);

        let (res1, res2) = (client.send(req1), client.send(req2)).join().await;
        let (body1, body2) = (read_to_end(res1), read_to_end(res2)).join().await;

        let body1 = String::from_utf8(body1).unwrap();
        println!("{body1}");

        let body2 = String::from_utf8(body2).unwrap();
        println!("{body2}");
    })
}

async fn read_to_end(mut res: Response) -> Vec<u8> {
    let mut body = vec![];
    while let Some(chunk) = res.next_chunk().await {
        let chunk = chunk.unwrap();
        body.extend_from_slice(&chunk);
    }
    body
}

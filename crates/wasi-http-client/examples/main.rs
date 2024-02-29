use wasi_http_client::runtime::block_on;
use wasi_http_client::{Client, Method, Request};

fn main() {
    block_on(|reactor| async {
        let client = Client::new(reactor);
        let url = "https://example.com".parse().unwrap();
        let req = Request::new(Method::Get, url);
        let mut res = client.send(req).await;

        let mut output = vec![];
        while let Some(chunk) = res.next_chunk().await {
            let chunk = chunk.unwrap();
            output.extend_from_slice(&chunk);
        }

        dbg!(String::from_utf8(output).unwrap());
    })
}

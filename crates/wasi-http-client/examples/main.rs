use std::str::FromStr;

use wasi_http_client::runtime::Runtime;
use wasi_http_client::{Client, Method, Request, Url};

fn main() {
    Runtime::new().run(|reactor| async {
        let client = Client::new(reactor);
        let url = Url::from_str("https://example.com").unwrap();
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

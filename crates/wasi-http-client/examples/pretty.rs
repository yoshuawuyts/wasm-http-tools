use std::str::FromStr;

use wasi_http_client::runtime::Runtime;
use wasi_http_client::{Client, Method, Request, Url};

fn main() {
    let runtime = Runtime::new();
    runtime.run(|reactor| async {
        let client = Client::new(reactor);
        let url = Url::from_str("https://example.com").unwrap();
        let req = Request::new(Method::Get, url);
        let res = client.send(req).await;
        dbg!(res.response.headers().entries());
    });
}

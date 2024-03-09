use futures_concurrency::prelude::*;
use wasi_async_runtime::block_on;
use wasi_http_client::{Client, Method, Request, Response, Result};

fn main() -> Result<()> {
    block_on(|reactor| async {
        let client = Client::new(reactor);

        let a = async {
            let url = "https://example.com".parse().unwrap();
            let req = Request::new(Method::Get, url);
            let res = client.send(req).await?;

            let body = read_to_end(res).await;
            let body = String::from_utf8(body).unwrap();
            println!("{body}");
            Result::Ok(())
        };

        let b = async {
            let url = "https://example.com".parse().unwrap();
            let req = Request::new(Method::Get, url);
            let res = client.send(req).await?;

            let body = read_to_end(res).await;
            let body = String::from_utf8(body).unwrap();
            println!("{body}");
            Result::Ok(())
        };

        (a, b).try_join().await?;
        Ok(())
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

use reqwest::{Method, StatusCode};
use vercel_runtime::{run, Body, Error, Request, Response};

use data2img::{compress::compress, render::render};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match *req.method() {
        Method::GET => render(req).await,
        Method::POST => compress(req).await,
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("Method Not Allowed".to_string()))?),
    }
}

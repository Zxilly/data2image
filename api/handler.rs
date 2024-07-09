use reqwest::{Method, StatusCode};
use vercel_runtime::{Body, Error, Request, Response, run};

use data2img::{compress::compress, render::render};
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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


use reqwest::Method;
use reqwest::StatusCode;
use vercel_runtime::{run, service_fn, Error, Request, Response, ResponseBody};

use data2img::{compress::compress, render::render};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn: fn(Request) -> _ = handler;
    run(service_fn::<_, (Request,)>(handler_fn)).await
}

async fn handler(req: Request) -> Result<Response<ResponseBody>, Error> {
    match *req.method() {
        Method::GET => render(req).await,
        Method::POST => compress(req).await,
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(ResponseBody::from("Method Not Allowed"))?),
    }
}

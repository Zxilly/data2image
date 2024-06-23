use data2img::DataType;

use std::collections::HashMap;

use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let has_data = hash_query.contains_key("data");
    if !has_data {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("data is required".to_string()))?);
    }
    let data = hash_query.get("data").unwrap();

    let typ: DataType = match url.path_segments() {
        None => DataType::Text,
        Some(p) => {
            let parts = p.collect::<Vec<&str>>();
            match parts.len() {
                0 => DataType::Text,
                1 => match parts[0] {
                    "brotli" => DataType::Brotli,
                    "deflate" => DataType::Deflate,
                    "zstd" => DataType::Zstd,
                    _ => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::Text("Invalid data type".to_string()))?)
                    }
                },
                _ => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::Text("Too many path args".to_string()))?)
                }
            };
            unreachable!()
        }
    };

    let result = data2img::decode(data.to_string(), typ).await;

    match result {
        Ok(s) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "image/svg+xml")
            .body(Body::Text(s))?),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text(e))?),
    }
}

use async_compression::Level;
use async_compression::tokio::write::ZstdEncoder;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use tokio::io::AsyncWriteExt;
use url_builder::URLBuilder;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

use crate::ZSTD_DICT;

#[cfg(debug_assertions)]
const PROTOCOL: &str = "http";

#[cfg(not(debug_assertions))]
const PROTOCOL: &str = "https";

#[cfg(debug_assertions)]
const HOST: &str = "localhost:3000";

#[cfg(not(debug_assertions))]
const HOST: &str = "bin2image.zxilly.dev";

pub async fn compress(req: Request) -> Result<Response<Body>, Error> {
    if req.body().is_empty() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("Missing data".to_string()))?);
    }
    
    let size = req.body().len();
    if size > 1024 * 100 {
        return Ok(Response::builder()
            .status(StatusCode::PAYLOAD_TOO_LARGE)
            .body(Body::Text("Payload too large, no more than 100KB".to_string()))?);
    }

    let data = req.body().to_vec();
    if String::from_utf8(data.clone()).is_err() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("Data is not utf8".to_string()))?);
    }

    let mut encoder = ZstdEncoder::with_dict(
        Vec::new(), Level::Best, ZSTD_DICT,
    ).unwrap();

    encoder.write_all(&data).await?;
    encoder.shutdown().await?;

    let compressed = encoder.into_inner();

    let base64 = BASE64_STANDARD.encode(compressed);

    let mut ret_uri = URLBuilder::new();
    ret_uri.set_protocol(PROTOCOL)
        .set_host(HOST)
        .add_param("type", "zstd-dict")
        .add_param("data", base64.as_str());
    let ret_uri = ret_uri.build();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::Text(ret_uri))?)
}
use async_compression::tokio::write::ZstdEncoder;
use async_compression::Level;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use tokio::io::AsyncWriteExt;
use url::Url;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

use crate::ZSTD_DICT;

#[cfg(debug_assertions)]
const BASE: &str = "http://localhost:3000";

#[cfg(not(debug_assertions))]
const BASE: &str = "https://bin2image.zxilly.dev";

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
            .body(Body::Text(
                "Payload too large, no more than 100KB".to_string(),
            ))?);
    }

    let data = req.body().to_vec();
    if String::from_utf8(data.clone()).is_err() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("Data is not utf8".to_string()))?);
    }

    let mut encoder = ZstdEncoder::with_dict(Vec::new(), Level::Best, ZSTD_DICT).unwrap();

    encoder.write_all(&data).await?;
    encoder.shutdown().await?;

    let compressed = encoder.into_inner();

    let base64 = BASE64_STANDARD.encode(compressed);

    let mut url = Url::parse(BASE)?;
    url.query_pairs_mut()
        .append_pair("type", "zstd-dict")
        .append_pair("data", base64.as_str());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .header(
            "X-Compressed-Ratio",
            format!("{:.2}%", (base64.len() as f64 / size as f64) * 100.0),
        )
        .body(Body::Text(url.to_string()))?)
}

use std::collections::HashMap;
use reqwest::StatusCode;
use url::Url;
use vercel_runtime::{Body, Error, Request, Response};
use crate::DataType;

pub async fn render(req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let has_data = hash_query.contains_key("data");
    let has_url = hash_query.contains_key("url");
    if !has_data && !has_url {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("Missing data or url parameter".to_string()))?);
    }
    let data = match hash_query.get("data") {
        None => {
            let url = hash_query.get("url").unwrap();
            let response = reqwest::get(url).await?;
            response.text().await?
        }
        Some(d) => d.to_string(),
    };

    let typ = match hash_query.get("type") {
        None => DataType::Text,
        Some(t) => match t.as_str() {
            "brotli" => DataType::Brotli,
            "deflate" => DataType::Deflate,
            "zstd" => DataType::Zstd,
            "zstd-dict" => DataType::ZstdDict, // "zstd-dict" => "ZstdDict
            "gzip" => DataType::Gzip,
            "text" => DataType::Text,
            _ => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::Text(format!("Unknown data type: {}", t)))?)
            }
        },
    };

    let result = crate::decode(data, typ).await;

    match result {
        Ok(s) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "image/svg+xml")
            .header(
                "Vercel-CDN-Cache-Control",
                "maxage=31536000, public, stale-while-revalidate",
            )
            .header(
                "CDN-Cache-Control",
                "maxage=31536000, public, stale-while-revalidate",
            )
            .header(
                "Cache-Control",
                "maxage=31536000, public, stale-while-revalidate",
            )
            .body(Body::Text(s))?),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text(e))?),
    }
}
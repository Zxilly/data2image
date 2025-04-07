use async_compression::tokio::write::ZstdEncoder;
use async_compression::Level;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use rquickjs::{async_with, AsyncContext, AsyncRuntime, Function, Module, Object};
use tokio::io::AsyncWriteExt;
use url::Url;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

use crate::ZSTD_DICT;

#[cfg(debug_assertions)]
const BASE: &str = "http://localhost:3000";

#[cfg(not(debug_assertions))]
const BASE: &str = "https://bin2image.zxilly.dev";

const SVGO: &str = include_str!(concat!(env!("OUT_DIR"), "/svgo.browser.js"));

async fn optimize_svg(svg: String) -> Result<String, Error> {
    let runtime = AsyncRuntime::new()?;
    let context = AsyncContext::full(&runtime).await?;

    if SVGO.is_empty() {
        panic!("SVGO is empty");
    }

    let result = async_with!(context => |ctx| {
        let module = Module::declare(ctx, "svgo", SVGO).expect("failed to declare module");
        let (module, wait) = module.eval().expect("failed to eval module");
        wait.finish::<()>().expect("failed to finish module");

        let optimize = module.get::<_, Function>("optimize").expect("failed to get optimize");

        let result_obj: Object = optimize.call((svg,)).expect("failed to call optimize");

        let data: String = result_obj.get::<_, String>("data").expect("failed to get data");

        return data;
    })
    .await;

    Ok(result)
}

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

    let mut data = req.body().to_vec();
    if String::from_utf8(data.clone()).is_err() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::Text("Data is not utf8".to_string()))?);
    }

    if req.headers().contains_key("X-Optimize-Svg") {
        data = optimize_svg(String::from_utf8(data).unwrap())
            .await?
            .into_bytes();
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

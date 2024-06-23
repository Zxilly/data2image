use std::pin::Pin;

use async_compression::tokio::bufread::{BrotliDecoder, DeflateDecoder, GzipDecoder, ZstdDecoder};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use tokio::io::{AsyncRead, AsyncReadExt};

pub enum DataType {
    Brotli,
    Deflate,
    Gzip,
    Zstd,
    Text,
}

pub async fn decode(data: String, typ: DataType) -> Result<String, String> {
    if let DataType::Text = typ {
        return Ok(data);
    }

    // decode base64
    let bin = match BASE64_STANDARD.decode(data.clone()) {
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };
    let bin = bin.as_slice();

    let mut decoder: Pin<Box<dyn AsyncRead>> = match typ {
        DataType::Brotli => Box::pin(BrotliDecoder::new(bin)),
        DataType::Deflate => Box::pin(DeflateDecoder::new(bin)),
        DataType::Gzip => Box::pin(GzipDecoder::new(bin)),
        DataType::Zstd => Box::pin(ZstdDecoder::new(bin)),
        _ => unreachable!(),
    };

    let mut data: Vec<u8> = vec![];
    let decoded = decoder.read_to_end(&mut data).await;

    if let Err(e) = decoded {
        return Err(format!("Failed to decode data: {}", e));
    }

    match String::from_utf8(data) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

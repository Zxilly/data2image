use std::pin::Pin;

use async_compression::tokio::bufread::{BrotliDecoder, DeflateDecoder, ZstdDecoder};
use tokio::io::{AsyncRead, AsyncReadExt};

pub enum DataType {
    Brotli,
    Deflate,
    Zstd,
    Text,
}

pub async fn decode(data: String, typ: DataType) -> Result<String, String> {
    let bin = data.as_bytes();

    let mut decoder: Pin<Box<dyn AsyncRead>> = match typ {
        DataType::Brotli => Box::pin(BrotliDecoder::new(bin)),
        DataType::Deflate => Box::pin(DeflateDecoder::new(bin)),
        DataType::Zstd => Box::pin(ZstdDecoder::new(bin)),
        DataType::Text => return Ok(data),
    };

    let mut data: Vec<u8> = vec![];
    let decoded = decoder.read_to_end(&mut data).await;

    if let Err(e) = decoded {
        return Err(e.to_string());
    }

    match String::from_utf8(data) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

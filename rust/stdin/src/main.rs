use anyhow::Result;
use bytes::{BufMut, BytesMut};
use reqwest::{Body, Client};
use tokio::io;
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

use core::convert::Infallible;

#[tokio::main]
async fn main() -> Result<()> {
    let mut buffer = BytesMut::with_capacity(1024 * 1024 * 5);
    let stdin = FramedRead::new(io::stdin(), BytesCodec::new());
    let mut stdin = stdin.map(|i| i.map(|bytes| bytes.freeze()));
    while let Some(bytes) = stdin.try_next().await? {
        buffer.put(bytes);
    }

    println!("buffer: {}", buffer.len());

    let stream = async_stream::stream! {
        while !buffer.is_empty() {
            let out = if buffer.len() > 8192 {
                buffer.split_to(8192)
            } else {
                buffer.split_to(buffer.len())
            };
            yield Ok::<_, Infallible>(out.freeze());
        }
    };

    // let stream = futures::stream::once(async { Ok::<_, Infallible>(buffer) });

    let client = Client::new();
    let body = Body::wrap_stream(stream);

    Ok(())
}

// use anyhow::Result;
use bytes::{BufMut, BytesMut};
use reqwest::{Body, Client};
use tokio::io;
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

use core::convert::Infallible;

/*
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
struct ByteStream {
    buffer: BytesMut,
}

impl ByteStream {
    pub fn new(buffer: BytesMut) -> Self {
        Self { buffer }
    }
}

impl Stream for ByteStream {
    type Item = Bytes;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.buffer.is_empty() {
            Poll::Ready(None)
        } else {
            let buf_len = self.buffer.len();
            let out = if buf_len > 8192 {
                self.buffer.split_to(8192)
            } else {
                self.buffer.split_to(buf_len)
            };
            Poll::Ready(Some(Ok::<_, Infallible>(out.freeze())))
        }
    }
}
*/

// https://users.rust-lang.org/t/how-to-use-bytes-bytesmut-correctly/35817/3
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = BytesMut::with_capacity(1024 * 1024 * 5);
    let stdin = FramedRead::new(io::stdin(), BytesCodec::new());
    let mut stdin = stdin.map(|i| i.map(|bytes| bytes.freeze()));
    while let Some(bytes) = stdin.try_next().await? {
        buffer.put(bytes);
    }
    println!("buffer: {}", buffer.len());

    let stream = futures::stream::once(async { Ok::<_, Infallible>(buffer) });

    //println!("buffer len: {}", buffer.len());
    let client = Client::new();
    let body = Body::wrap_stream(stream);

    Ok(())
}

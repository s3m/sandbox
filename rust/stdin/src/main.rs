use anyhow::Result;
use bytes::{BufMut, Bytes, BytesMut};
//use reqwest::{Body, Client};
use tokio::io;
use tokio::stream::{Stream, StreamExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ByteStream<R: io::AsyncRead>(pub R);

impl<R: io::AsyncRead> Stream for ByteStream<R> {
    type Item = Bytes;

    fn poll_next(self: Pin<&'_ mut Self>, ctx: &'_ mut Context) -> Poll<Option<Self::Item>> {
        let mut buf = BytesMut::with_capacity(8192);
        this.0
            .poll_read_buf(ctx, &mut buf) // calls `buf.advance_mut(n)()`
            .map(|it| match it {
                Ok(n) if n != 0 => Some(buf.freeze()),
                _ => None,
            })
    }
}

// https://users.rust-lang.org/t/how-to-use-bytes-bytesmut-correctly/35817/3
#[tokio::main]
async fn main() -> Result<()> {
    let mut buffer = BytesMut::with_capacity(1024 * 1024 * 5);
    let stdin = FramedRead::new(io::stdin(), BytesCodec::new());
    let mut stdin = stdin.map(|i| i.map(|bytes| bytes.freeze()));
    while let Some(bytes) = stdin.try_next().await? {
        buffer.put(bytes);
    }
    println!("buffer: {}", buffer.len());

    //let client = Client::new();
    //let body = Body::wrap_stream(x);

    Ok(())
}

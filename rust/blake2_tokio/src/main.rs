use futures::stream::TryStreamExt;
use ring::digest::{Context, SHA256};
use std::error::Error;
use std::fmt::Write;
use std::time::Instant;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let checksum = blake("/tmp/wine.json").await.unwrap();
    println!("blake: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = sha256_digest("/tmp/wine.json").await.unwrap();
    println!("sha256: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

async fn blake(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let mut stream = FramedRead::new(file, BytesCodec::new());
    let mut hasher = blake2s_simd::State::new();
    while let Some(bytes) = stream.try_next().await? {
        hasher.update(&bytes);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

async fn sha256_digest(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let mut stream = FramedRead::new(file, BytesCodec::new());
    let mut context = Context::new(&SHA256);
    while let Some(bytes) = stream.try_next().await? {
        context.update(&bytes);
    }
    let digest = context.finish();
    Ok(write_hex_bytes(digest.as_ref()))
}

pub fn write_hex_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}

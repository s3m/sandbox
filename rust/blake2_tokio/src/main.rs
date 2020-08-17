use futures::stream::TryStreamExt;
use ring::digest::{Context, SHA256};
use std::error::Error;
use std::fmt::Write;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio::prelude::*;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let checksum = blake("/tmp/wine.json").await.unwrap();
    println!("blake: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = blake_buffer("/tmp/wine.json").await.unwrap();
    println!("blake: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = blake3("/tmp/wine.json").await.unwrap();
    println!("blake3: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = sha256_digest("/tmp/wine.json").await.unwrap();
    println!("sha256: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = sha256_digest_bufreader("/tmp/wine.json").await.unwrap();
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

async fn blake3(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let mut stream = FramedRead::new(file, BytesCodec::new());
    let mut hasher = blake3::Hasher::new();
    while let Some(bytes) = stream.try_next().await? {
        hasher.update(&bytes);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

async fn blake_buffer(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake2s_simd::State::new();
    let mut buf: [u8; 8192] = [0; 8192];
    while let Ok(size) = reader.read(&mut buf[..]).await {
        if size == 0 {
            break;
        }
        hasher.update(&buf[0..size]);
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

async fn sha256_digest_bufreader(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut context = Context::new(&SHA256);
    let mut buf: [u8; 8192] = [0; 8192];
    while let Ok(size) = reader.read(&mut buf[..]).await {
        if size == 0 {
            break;
        }
        context.update(&buf[0..size]);
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

use ring::digest::{Context, SHA256};
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let checksum = blake2("/tmp/wine.json").unwrap();
    println!("blake2: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = sha256_digest("/tmp/wine.json").unwrap();
    println!("sha256: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = blake3("/tmp/wine.json").unwrap();
    println!("blake3: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = blake3_rayon("/tmp/wine.json").unwrap();
    println!("blake3 rayon: {}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

pub fn blake2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake2s_simd::State::new();
    loop {
        let consumed = {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                break;
            }
            hasher.update(buffer);
            buffer.len()
        };
        reader.consume(consumed);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn sha256_digest(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut context = Context::new(&SHA256);

    loop {
        let consummed = {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                break;
            }
            context.update(buffer);
            buffer.len()
        };
        reader.consume(consummed);
    }

    let digest = context.finish();

    Ok(write_hex_bytes(digest.as_ref()))
}

pub fn blake3(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buf: [u8; 8192] = [0; 8192]; //chunk size (8K, 65536, etc)

    while let Ok(size) = reader.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        hasher.update(&buf[0..size]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn blake3_rayon(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buf: [u8; 65536] = [0; 65536];

    while let Ok(size) = reader.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        hasher.update_with_join::<blake3::join::RayonJoin>(&buf[0..size]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn write_hex_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}

use ring::digest::{Context, SHA256};
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let checksum = blake2("/tmp/wine.json").unwrap();
    println!("{}", checksum);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();
    let checksum = sha256_digest("/tmp/wine.json").unwrap();
    println!("{}", checksum);
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

pub fn write_hex_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}

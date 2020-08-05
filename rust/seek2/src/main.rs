use std::env;
use std::fs::metadata;
//use std::io::{Read, Write};
use std::process;

use std::io::SeekFrom;
use tokio::fs::File;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("missing arguments: file, chunks");
        process::exit(1);
    }
    let file_path = &args[1];
    let slices = &args[2].parse::<u64>().unwrap();
    let fsize = metadata(file_path).map(|m| m.len()).unwrap();
    let chunk_size = (fsize / slices) as usize;
    println!("file size: {}, chunk size: {}", fsize, chunk_size);
    let mut position = 0;
    for _ in 0..*slices {
        println!("seek:{}", position);
        position = position + chunk_size;
    }
    let mut file = File::open(&file_path).await.unwrap();
    let mut i: usize = 0;
    let mut buf = [0u8; 1024];
    loop {
        file.read(&mut buf).await.unwrap();
        file.seek(SeekFrom::Start(0)).await.unwrap();
        let mut f = File::create(&format!("/tmp/chunk_{}", i)).await.unwrap();
        f.write_all(&buf).await.expect("Unable to write data");
        i += 1;
    }
}

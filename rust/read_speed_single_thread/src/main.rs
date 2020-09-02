use anyhow::Result;
use bytes::BytesMut;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::time::Instant;
use std::{env, process::exit};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const BUF_SIZE: usize = 1024 * 1024 * 10;

//#[tokio::main]
//async fn main() -> Result<()> {
fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing arguments: file");
        exit(1);
    }
    let file_path = &args[1];

    let mut file = fs::File::open(&file_path).unwrap();
    let mut count = 0;
    // let mut buffer = BytesMut::with_capacity(BUF_SIZE);
    let mut buf = [0; 1024 * 64];
    while let Ok(size) = file.read(&mut buf[..]) {
        println!("buf size: {}", buf.len());
        if size == 0 {
            break;
        }
        count += size;
        println!("{}", count);
        //if buffer.len() + size >= BUF_SIZE {
        //println!("{}", count);
        //}
    }
    println!("{}", count);
    println!("Elapsed: {:?}", now.elapsed());
    //    Ok(())
}

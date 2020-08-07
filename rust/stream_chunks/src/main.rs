use futures::stream::TryStreamExt;
use std::fs::metadata;
use std::io::SeekFrom;
use std::{env, error, process};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::prelude::*;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing arguments: file");
        process::exit(1);
    }
    let file_path = &args[1];
    let fsize = metadata(file_path).map(|m| m.len()).unwrap();
    let mut chunk_size = 10_485_760;
    println!(
        "file size: {}, chunk size: {}, parts: {}",
        fsize,
        chunk_size,
        fsize / chunk_size
    );
    let mut seek: u64 = 0;
    let mut parts: Vec<Vec<u64>> = Vec::new();
    while seek < fsize {
        if (fsize - seek) <= chunk_size {
            chunk_size = fsize % chunk_size;
        }
        println!(
            "seek: {}, chunk: {}, rem: {}",
            seek,
            chunk_size,
            fsize - seek
        );
        parts.push(vec![seek, chunk_size]);
        seek += chunk_size;
    }

    let mut file = File::open(&file_path).await.unwrap();

    for part in 0..parts.len() {
        println!(
            "part: {}, seek: {}, chunk: {}",
            part, parts[part][0], parts[part][1]
        );
        match read_file(&mut file, parts[part][0], parts[part][1], part).await {
            Ok(_) => (), //println!("---\n{:#?}\n---", rs),
            Err(e) => eprintln!("{}", e),
        };
    }
}

async fn read_file(
    file: &mut File,
    seek: u64,
    chunk: u64,
    part: usize,
) -> Result<(), Box<dyn error::Error>> {
    file.seek(SeekFrom::Start(seek)).await?;
    let file = file.take(chunk);
    let mut stream = FramedRead::new(file, BytesCodec::new());
    let mut f = File::create(&format!("/tmp/chunks/chunk_{}", part)).await?;
    while let Some(bytes) = stream.try_next().await? {
        f.write_all(&bytes).await?;
    }
    Ok(())
}

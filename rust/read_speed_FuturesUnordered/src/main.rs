use anyhow::Result;
use futures::stream::FuturesUnordered;
use std::fs::metadata;
use std::io::SeekFrom;
use std::time::Instant;
use std::{env, process::exit};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing arguments: file");
        exit(1);
    }
    let file_path = &args[1];
    let fsize = metadata(&file_path).map(|m| m.len()).unwrap();
    let mut chunk_size = 10_485_760;
    println!(
        "file size: {}, chunk size: {}, parts: {}",
        fsize,
        chunk_size,
        fsize / chunk_size
    );
    let mut seek: u64 = 0;
    let mut parts: Vec<(u64, u64)> = Vec::new();
    while seek < fsize {
        if (fsize - seek) <= chunk_size {
            chunk_size = fsize % chunk_size;
        }
        println!("seek: {}, chunk: {}", seek, chunk_size,);
        parts.push((seek, chunk_size));
        seek += chunk_size;
    }

    let mut tasks = FuturesUnordered::new();
    for (pos, part) in parts.iter().enumerate() {
        tasks.push(read_file(&file_path, part.0, part.1, pos));

        // limit to only 4 tasks concurrent
        if tasks.len() == 4 {
            if let Some(t) = tasks.next().await {
                println!("{:#?}", t.unwrap());
            }
        }
    }

    while let Some(t) = tasks.next().await {
        println!("{:#?}", t.unwrap());
    }
    println!("Elapsed: {:?}", now.elapsed());
    Ok(())
}

async fn read_file(path: &str, seek: u64, chunk: u64, part: usize) -> Result<String> {
    let mut file = File::open(&path).await?;
    file.seek(SeekFrom::Start(seek)).await?;
    let file = file.take(chunk);
    let mut stream = FramedRead::with_capacity(file, BytesCodec::new(), 1024 * 64);
    let mut count = 0;
    while let Some(bytes) = stream.try_next().await? {
        count += bytes.len();
        // do something here, upload/stream the file PUT/POST
        // no CPU intensive so maybe spawn will not help much
    }
    Ok(format!("part: {}, size: {}", part, count.to_string()))
}

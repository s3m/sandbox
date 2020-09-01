use anyhow::Result;
use futures::future::join_all;
use std::fs::metadata;
use std::io::SeekFrom;
use std::sync::Arc;
use std::time::Instant;
use std::{env, process::exit};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;
use tokio::sync::Semaphore;
use tokio::task;
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
        println!("seek: {}, chunk: {}", seek, chunk_size,);
        parts.push(vec![seek, chunk_size]);
        seek += chunk_size;
    }

    let mut tasks = Vec::new();
    let sem = Arc::new(Semaphore::new(4));
    for part in 0..parts.len() {
        let file = file_path.clone();
        let p = parts.clone();
        let permit = Arc::clone(&sem).acquire_owned().await;
        tasks.push(task::spawn(async move {
            let _permit = permit;
            println!(
                "read part: {}, seek: {}, chunk: {}",
                part, p[part][0], p[part][1]
            );
            match read_file(&file, p[part][0], p[part][1], part).await {
                Ok(rs) => println!("{}", rs),
                Err(e) => eprintln!("{}", e),
            };
        }));
    }
    join_all(tasks).await;
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
    }
    Ok(format!("part: {}, size: {}", part, count.to_string()))
}

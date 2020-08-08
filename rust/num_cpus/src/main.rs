use futures::stream::TryStreamExt;
use futures::stream::{futures_unordered::FuturesUnordered, StreamExt};
use num_cpus;
use std::fs::metadata;
use std::io::SeekFrom;
use std::sync::Arc;
use std::time::Instant;
use std::{env, error, process};
use tokio::fs::File;
use tokio::prelude::*;
use tokio::sync::Semaphore;
use tokio::task;
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

    let now = Instant::now();
    // cpu cores * 2
    let workers = num_cpus::get() * 2;
    println!("Number of workers: {}", workers);

    let tasks = FuturesUnordered::new();
    let sem = Arc::new(Semaphore::new(workers));
    for part in 0..parts.len() {
        let file = file_path.clone();
        let p = parts.clone();
        let permit = Arc::clone(&sem).acquire_owned().await;
        tasks.push(task::spawn(async move {
            let _permit = permit;
            println!(
                "part: {}, seek: {}, chunk: {}",
                part, p[part][0], p[part][1]
            );
            match read_file(&file, p[part][0], p[part][1], part).await {
                Ok(_) => (), //println!("---\n{:#?}\n---", rs),
                Err(e) => eprintln!("{}", e),
            };
        }));
    }
    tasks.for_each(|_| async { () }).await;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

async fn read_file(
    path: &str,
    seek: u64,
    chunk: u64,
    part: usize,
) -> Result<(), Box<dyn error::Error>> {
    let mut file = File::open(&path).await?;
    file.seek(SeekFrom::Start(seek)).await?;
    let file = file.take(chunk);
    let mut stream = FramedRead::new(file, BytesCodec::new());
    let mut f = File::create(&format!("/tmp/chunks/chunk_{}", part)).await?;
    while let Some(bytes) = stream.try_next().await? {
        f.write_all(&bytes).await?;
    }
    Ok(())
}

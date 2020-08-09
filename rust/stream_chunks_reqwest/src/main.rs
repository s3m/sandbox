use futures::stream::FuturesUnordered;
use reqwest::{Body, Client};
use std::fs::metadata;
use std::io::SeekFrom;
use std::time::Instant;
use std::{env, error, process};
use tokio::fs::File;
use tokio::prelude::*;
use tokio::stream::StreamExt;
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
    let workers = if num_cpus::get() * 2 > 4 {
        3
    } else {
        num_cpus::get()
    };
    println!("Number of workers: {}", workers);

    let mut tasks = FuturesUnordered::new();
    for part in 0..parts.len() {
        let file = file_path.clone();
        let p = parts.clone();
        tasks.push(task::spawn(async move {
            println!(
                "part: {}, seek: {}, chunk: {}",
                part, p[part][0], p[part][1]
            );
            match read_file(&file, p[part][0], p[part][1], part).await {
                Ok(rs) => println!("---\n{:#?}\n---", rs),
                Err(e) => eprintln!("{}", e),
            };
        }));
        if tasks.len() == workers {
            tasks.next().await;
        }
    }
    // This loop is how to wait for all the elements in a `FuturesUnordered<T>`
    // to complete. `_item` is just the unit tuple, `()`, because we did not
    // return anything
    while let Some(_item) = tasks.next().await {}

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

async fn read_file(
    path: &str,
    seek: u64,
    chunk: u64,
    part: usize,
) -> Result<String, Box<dyn error::Error>> {
    let mut file = File::open(&path).await?;
    file.seek(SeekFrom::Start(seek)).await?;
    let file = file.take(chunk);
    let stream = FramedRead::new(file, BytesCodec::new());
    let client = Client::new();
    let body = Body::wrap_stream(stream);
    let request = client.put("https://httpbin.org/put").body(body);
    let rs = request.send().await?;
    Ok(rs.text().await?)
}

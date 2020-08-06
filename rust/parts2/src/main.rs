use reqwest::{Body, Client};
use std::env;
use std::io::SeekFrom;
use std::process;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing arguments: file");
        process::exit(1);
    }
    let file_path = &args[1];
    let mut file = File::open(&file_path).await.unwrap();
    file.seek(SeekFrom::Start(10)).await.unwrap();
    let file = file.take(10);
    let stream = FramedRead::new(file, BytesCodec::new());
    let client = Client::new();
    let body = Body::wrap_stream(stream);
    let request = client.put("https://httpbin.org/put").body(body);
    let rs = request.send().await.unwrap();
    println!("---\n{:#?}\n---", rs.text().await.unwrap());
}

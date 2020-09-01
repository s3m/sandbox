use anyhow::Result;
// use bytes::{BufMut, BytesMut};
use indicatif::{ProgressBar, ProgressStyle};
/*
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
*/
use tokio::io;
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "\u{2801}", "\u{2802}", "\u{2804}", "\u{2840}", "\u{2880}", "\u{2820}", "\u{2810}",
                "\u{2808}", "",
            ])
            .template("{spinner:.green}  {msg}"),
    );

    //let mut buffer = BytesMut::with_capacity(1024 * 1024 * 5);
    let mut stdin = FramedRead::with_capacity(io::stdin(), BytesCodec::new(), 1024 * 64);
    let mut count: usize = 0;
    while let Some(bytes) = stdin.try_next().await? {
        count += bytes.len();
        pb.set_message(&bytesize::to_string(count as u64, true));
    }
    pb.finish();

    println!("Elapsed: {:?}, bytes: {}", now.elapsed(), count);
    Ok(())
}

/*
fn main() {
    let now = Instant::now();
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "\u{2801}", "\u{2802}", "\u{2804}", "\u{2840}", "\u{2880}", "\u{2820}", "\u{2810}",
                "\u{2808}", "",
            ])
            .template("{spinner:.green}  {msg}"),
    );
    let stdin = std::io::stdin().as_raw_fd();
    let mut f = unsafe { File::from_raw_fd(stdin) };
    let mut buf = [0; 65536];
    let mut count: usize = 0;
    while let Ok(size) = f.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        count += size;
        pb.set_message(&bytesize::to_string(count as u64, true));
    }
    pb.finish();
    println!("Elapsed: {:?}", now.elapsed());
}
*/

use std::fs;
use std::io::prelude::*;
use std::time::Instant;
use std::{env, process::exit};

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
    let mut buf = [0; 1024 * 64];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        count += size;
        println!("{}", count);
    }
    println!("{}", count);
    println!("Elapsed: {:?}", now.elapsed());
    //    Ok(())
}

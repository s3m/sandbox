use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing arguments: file");
        process::exit(1);
    }
    let file_path = &args[1];
    let file = File::open(&file_path).unwrap();

    let mut count = 1;
    loop {
        let mut reader = BufReader::new(&file);
        if reader.fill_buf().unwrap().is_empty() {
            break;
        }
        let mut reader = reader.take(10_485_760);
        let mut f = File::create(&format!("/tmp/chunk_{}", count)).unwrap();
        loop {
            let consummed = {
                let buffer = reader.fill_buf().unwrap();
                if buffer.is_empty() {
                    break;
                }
                // do something here with buffer
                // client.put(url).headers(headers).body(Body::from(&buffer))
                f.write_all(&buffer).expect("Unable to write data");
                buffer.len()
            };
            reader.consume(consummed);
        }
        count += 1;
    }
}

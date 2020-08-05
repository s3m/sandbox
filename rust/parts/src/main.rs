use std::env;
use std::error::Error;
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("missing arguments: file, chunks");
        process::exit(1);
    }
    let file_path = &args[1];
    let slices = &args[2].parse::<u64>().unwrap();
    let fsize = metadata(file_path).map(|m| m.len()).unwrap();
    let chunk_size = (fsize / slices) as usize;
    println!("file size: {}, chunk size: {}", fsize, chunk_size);
    let file = File::open(&file_path).unwrap();
    let mut i = 1;
    loop {
        let rs = match read_chunk(&file, chunk_size, i) {
            Ok(rs) => rs,
            Err(e) => {
                eprint!("{}", e);
                process::exit(1)
            }
        };
        if rs == 0 {
            break;
        }
        i += 1;
    }
}

pub fn read_chunk(file: &File, chunk: usize, i: u64) -> Result<usize, Box<dyn Error>> {
    let mut reader = BufReader::new(file);
    let mut length: usize = 0;
    let mut f = File::create(&format!("/tmp/chunk_{}", i)).unwrap();
    loop {
        let consummed = {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                break;
            }
            // do something here with buffer
            // client.put(url).headers(headers).body(Body::from(&buffer))
            f.write_all(&buffer).expect("Unable to write data");
            buffer.len()
        };
        length += consummed;
        reader.consume(consummed);
        if length >= chunk {
            break;
        }
    }
    Ok(length)
}

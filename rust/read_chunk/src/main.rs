use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{BufReader, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing arguments: file");
        process::exit(1);
    }
    let file_path = &args[1];

    if args.len() >= 3 {
        let mut file = File::open(&file_path).unwrap();
        file.seek(SeekFrom::Start(79279290)).unwrap();
        let handler = file.take(28);
        my_reader(handler);
    } else {
        let file = File::open(&file_path).unwrap();
        my_reader(file);
    }
}

fn my_reader<R: Read>(input: R) {
    let mut reader = BufReader::new(input);
    let mut f = File::create(&format!("/tmp/chunks/chunk0_{}", 1)).unwrap();
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
}

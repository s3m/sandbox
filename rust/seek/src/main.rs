use std::env;
use std::fs::metadata;
use std::fs::File;
use std::io::{Read, Write};
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
    let mut position = 0;
    for _ in 0..*slices {
        println!("seek:{}", position);
        position = position + chunk_size;
    }
    let mut file = File::open(&file_path).unwrap();
    let mut i = 0;
    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = std::io::Read::by_ref(&mut file)
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)
            .unwrap();
        if n == 0 {
            break;
        }
        let mut f = File::create(&format!("/tmp/chunk_{}", i)).unwrap();
        f.write_all(&chunk).expect("Unable to write data");
        i += 1;
    }
}

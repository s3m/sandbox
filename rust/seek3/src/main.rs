use std::env;
use std::fs::metadata;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("missing arguments: file, chunks");
        process::exit(1);
    }
    let file_path = &args[1];
    //let slices = &args[2].parse::<u64>().unwrap();
    let fsize = metadata(file_path).map(|m| m.len()).unwrap();
    let mut chunk_size = 10_485_760;
    println!(
        "file size: {}, chunk size: {}, parts: {}",
        fsize,
        chunk_size,
        fsize / chunk_size
    );
    let mut seek: u64 = 0;
    let mut vec: Vec<Vec<u64>> = Vec::new();
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
        vec.push(vec![seek, chunk_size]);
        seek += chunk_size;
    }
    //println!("vec: {:#?}", vec);
    println!("{}", vec.len());
}

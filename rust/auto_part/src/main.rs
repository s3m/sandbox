use bytesize::ByteSize;
const MAX_PARTS: u64 = 10000;

fn main() {
    let fsize: u64 = 322_122_547_200;
    let mut chunk_size = 10_485_760;

    let mut parts = fsize / chunk_size;
    println!(
        "parts: {}, chunk size for 10000 parts: {}",
        parts,
        ByteSize(fsize / MAX_PARTS)
    );

    while parts > MAX_PARTS {
        chunk_size = chunk_size * 2;
        parts = fsize / chunk_size;
        println!("parts: {}, chunk size: {}", parts, ByteSize(chunk_size));
    }
}

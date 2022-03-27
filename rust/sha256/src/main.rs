use ring::{
    digest,
    digest::{Context, SHA256},
    hmac,
};

fn main() {
    let x = digest::digest(&digest::SHA256, b"");
    println!("{:#?}", x);
}

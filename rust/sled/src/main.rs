use std::{thread, time};

fn main() {
    // this directory will be created if it does not exist
    let path = "/tmp/yy";

    // works like std::fs::open
    let db = sled::Config::new()
        .temporary(true)
        .path(path)
        .open()
        .unwrap();

    // key and value types can be `Vec<u8>`, `[u8]`, or `str`.
    let key = "my key";

    // `generate_id`
    let value = db.generate_id().unwrap().to_be_bytes();

    dbg!(
        db.insert(key, &value).unwrap(), // as in BTreeMap::insert
        db.get(key).unwrap(),            // as in BTreeMap::get
        db.remove(key).unwrap(),         // as in BTreeMap::remove
    );

    thread::sleep(time::Duration::new(5, 0));
}

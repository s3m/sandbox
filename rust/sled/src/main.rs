use serde::{Deserialize, Serialize};
use serde_cbor::{de::from_mut_slice, to_vec};
use std::{thread, time};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Part {
    id: usize,
    foo: String,
    bar: String,
}

fn main() {
    // this directory will be created if it does not exist
    let path = "/tmp/yy";

    let db = sled::Config::new()
        .temporary(true)
        .path(path)
        .open()
        .unwrap();

    let mut uid = String::new();

    if let Ok(u) = db.get("uid") {
        if let Some(u) = u {
            if let Ok(u) = String::from_utf8(u.to_vec()) {
                println!("uid found...");
                uid = u;
            }
        }
    };
    if uid.is_empty() {
        println!("uid not found, create a new one");
        uid = "abc-123".to_string();
        db.insert("uid", uid.as_bytes()).unwrap();
    }

    if db.is_empty() {
        println!("db is empty");
    }

    println!("db size: {}", db.len());

    let part = Part {
        id: 1,
        foo: "foo".to_string(),
        bar: "bar".to_string(),
    };

    let encoded = to_vec(&part).unwrap();

    let parts = db.open_tree("parts").unwrap();
    if parts.is_empty() {
        println!("tree is empty");
    }

    parts.insert("1", encoded).unwrap();
    parts.flush().unwrap();

    if let Ok(x) = parts.get("1") {
        if let Some(mut u) = x {
            let decoded: Part = from_mut_slice(&mut u[..]).unwrap();
            println!("{:#?}", decoded);
        }
    }

    thread::sleep(time::Duration::new(10, 0));
}

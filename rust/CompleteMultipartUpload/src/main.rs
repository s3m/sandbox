use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};
use serde_xml_rs::to_string;

#[derive(Debug)]
pub struct Part {
    pub e_tag: String,
    pub part_number: isize,
}

impl Serialize for Part {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("ETag", &self.e_tag)?;
        map.serialize_entry("PartNumber", &self.part_number)?;
        map.end()
    }
}

#[derive(Debug)]
pub struct Parts {
    parts: Vec<Part>,
}

impl Serialize for Parts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = 1 + self.parts.len();
        let mut map = serializer.serialize_struct("CompleteMultipartUpload", len)?;
        for part in &self.parts {
            map.serialize_field("Part", part)?;
        }
        map.end()
    }
}

fn main() {
    let mut buffer = Vec::new();
    for i in 1..5 {
        let part = Part {
            e_tag: "abc".to_string(),
            part_number: i,
        };
        buffer.push(part);
    }
    let parts = Parts { parts: buffer };
    let x = to_string(&parts).unwrap();
    println!("{}", x); // | xmllint --format -
}

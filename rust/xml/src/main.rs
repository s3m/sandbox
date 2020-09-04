use serde::de::{value, Deserializer, SeqAccess, Visitor};
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fmt;

pub fn versions_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }
    deserializer.deserialize_any(StringOrVec)
}

#[derive(Deserialize, Debug)]
pub struct ListVersionsResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "KeyMarker")]
    pub key_marker: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    pub is_truncated: Option<bool>,
    #[serde(rename = "Version", deserialize_with = "versions_deserializer")]
    pub versions: Vec<String>,
    //#[serde(rename = "DeleteMarker")]
    //pub delete_markers: Option<Vec<DeleteMarker>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Version {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub e_tag: Option<String>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,
    #[serde(rename = "Size")]
    pub size: u64,
}

fn main() {
    let s = r##"
<?xml version="1.0"?>
<ListVersionsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Name>mtp-versioning-fresh</Name>
  <Prefix/>
  <KeyMarker>key2</KeyMarker>
  <VersionIdMarker/>
  <MaxKeys>1000</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <Version>
    <Key>key3</Key>
    <VersionId>I5VhmK6CDDdQ5Pwfe1gcHZWmHDpcv7gfmfc29UBxsKU.</VersionId>
    <IsLatest>true</IsLatest>
    <LastModified>2009-12-09T00:19:04.000Z</LastModified>
    <ETag>"396fefef536d5ce46c7537ecf978a360"</ETag>
    <Size>217</Size>
    <Owner>
      <ID>75aa57f09aa0c8caeab4f8c24e99d10f8e7faeebf76c078efc7c6caea54ba06a</ID>
    </Owner>
    <StorageClass>STANDARD</StorageClass>
  </Version>
  <DeleteMarker>
    <Key>sourcekey</Key>
    <VersionId>qDhprLU80sAlCFLu2DWgXAEDgKzWarn-HS_JU0TvYqs.</VersionId>
    <IsLatest>true</IsLatest>
    <LastModified>2009-12-10T16:38:11.000Z</LastModified>
    <Owner>
      <ID>75aa57f09aa0c8caeab4f8c24e99d10f8e7faeebf76c078efc7c6caea54ba06a</ID>
    </Owner>
  </DeleteMarker>
  <Version>
    <Key>sourcekey</Key>
    <VersionId>wxxQ7ezLaL5JN2Sislq66Syxxo0k7uHTUpb9qiiMxNg.</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2009-12-10T16:37:44.000Z</LastModified>
    <ETag>"396fefef536d5ce46c7537ecf978a360"</ETag>
    <Size>217</Size>
    <Owner>
      <ID>75aa57f09aa0c8caeab4f8c24e99d10f8e7faeebf76c078efc7c6caea54ba06a</ID>
    </Owner>
    <StorageClass>STANDARD</StorageClass>
  </Version>
</ListVersionsResult>
    "##;

    let list: ListVersionsResult = from_str(s).unwrap();
    println!("{:#?}", list);
}

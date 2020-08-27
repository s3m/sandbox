use rusoto_core::ByteStream;
use rusoto_s3::{S3Client, S3};

use futures::{future, stream, Stream, StreamExt, TryStreamExt};

pub async fn upload_stream(
    client: S3Client,
    bucket: String,
    key: String,
    data_stream: impl Stream<Item = String>,
    buf_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let writer = StreamWriter::Init {
        client,
        bucket,
        key,
        buf_size: usize::max(buf_size, 1024 * 1024 * 5),
    };
    let result = data_stream
        .map(Some)
        .map(Ok)
        .chain(stream::once(future::ready(Ok(None))))
        .try_fold(writer, fold_fn)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

enum StreamWriter {
    Init {
        client: S3Client,
        bucket: String,
        key: String,
        buf_size: usize,
    },
    Uploading {
        bucket: String,
        key: String,
        client: S3Client,
        upload_id: String,
        buf_size: usize,
        part_count: i64,
        etags: Vec<String>,
        buffer: Vec<u8>,
    },
    Complete,
}

async fn fold_fn(
    writer: StreamWriter,
    x: Option<String>,
) -> Result<StreamWriter, Box<dyn std::error::Error>> {
    let writer = match writer {
        StreamWriter::Init {
            client,
            bucket,
            key,
            buf_size,
        } => {
            let mut req = rusoto_s3::CreateMultipartUploadRequest::default();
            req.bucket = bucket.clone();
            req.key = key.clone();

            let upload_result = client.create_multipart_upload(req).await?;
            let upload_id = upload_result.upload_id.unwrap();

            StreamWriter::Uploading {
                bucket,
                key,
                client,
                upload_id,
                buf_size,
                part_count: 1,
                buffer: Vec::with_capacity(buf_size),
                etags: Vec::new(),
            }
        }
        _ => writer,
    };

    match writer {
        StreamWriter::Uploading {
            bucket,
            key,
            client,
            upload_id,
            buf_size,
            part_count,
            mut buffer,
            mut etags,
        } => match x {
            Some(x) => {
                let mut bytes = x.into_bytes();
                match buffer.len() + bytes.len() >= buf_size {
                    true => {
                        let mut new_buf = Vec::with_capacity(buf_size);
                        let (prev_batch, next_batch) = bytes.split_at_mut(buf_size - buffer.len());

                        buffer.extend(prev_batch.iter());
                        new_buf.extend(next_batch.iter());

                        let mut req = rusoto_s3::UploadPartRequest::default();
                        req.bucket = bucket.clone();
                        req.key = key.clone();
                        req.upload_id = upload_id.clone();
                        req.body = Some(ByteStream::from(buffer));
                        req.part_number = part_count;

                        let etag = client.upload_part(req).await?.e_tag.unwrap();
                        etags.push(etag);

                        Ok(StreamWriter::Uploading {
                            bucket,
                            key,
                            client,
                            upload_id,
                            buffer: new_buf,
                            buf_size,
                            part_count: part_count + 1,
                            etags,
                        })
                    }
                    false => {
                        buffer.append(&mut bytes);
                        Ok(StreamWriter::Uploading {
                            bucket,
                            key,
                            client,
                            upload_id,
                            buffer,
                            buf_size,
                            part_count,
                            etags,
                        })
                    }
                }
            }
            None => {
                let mut req = rusoto_s3::UploadPartRequest::default();
                req.bucket = bucket.clone();
                req.key = key.clone();
                req.upload_id = upload_id.clone();
                req.body = Some(ByteStream::from(buffer));
                req.part_number = part_count;

                let upload_result = client.upload_part(req).await?;
                let etag = upload_result.e_tag.unwrap();
                etags.push(etag);

                let f = |(etag, part_number)| rusoto_s3::CompletedPart {
                    e_tag: Some(etag),
                    part_number: Some(part_number),
                };
                let completed_parts = rusoto_s3::CompletedMultipartUpload {
                    parts: Some(etags.into_iter().zip(1..).map(f).collect()),
                };

                let mut req = rusoto_s3::CompleteMultipartUploadRequest::default();
                req.bucket = bucket.clone();
                req.key = key.clone();
                req.upload_id = upload_id.clone();
                req.multipart_upload = Some(completed_parts);

                client.complete_multipart_upload(req).await?;

                Ok(StreamWriter::Complete)
            }
        },
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusoto_core::Region;

    const TEST_REGION: Region = Region::ApNortheast1;
    const TEST_BUCKET: &str = "test_bucket";  // Obviously create your s3 bucket first. 

    #[tokio::test]
    async fn empty_upload() {
        let example_stream = futures::stream::empty(); // Creates an empty file.

        let result = upload_stream(
            S3Client::new(TEST_REGION),
            String::from(TEST_BUCKET),
            String::from("empty_file.txt"),
            example_stream,
            5 * 1024 * 1024,
        )
        .await;

        if let Err(error) = result {
            panic!("{}", error);
        }
    }

    #[tokio::test]
    async fn simple_upload() {
        let example_stream = futures::stream::once(async { String::from("16") });

        let result = upload_stream(
            S3Client::new(TEST_REGION),
            String::from(TEST_BUCKET),
            String::from("simple_file.txt"),
            example_stream,
            5 * 1024 * 1024,
        )
        .await;

        if let Err(error) = result {
            panic!("{}", error);
        }
    }

    #[tokio::test]
    async fn large_upload() {
        use std::time::Duration;

        fn get_stream(start: i64, end: i64) -> impl Stream<Item = i64> {
            tokio::time::interval(Duration::from_millis(5)).scan(start, move |acc, _| {
                *acc += 1;
                match *acc {
                    x if x == end => future::ready(None),
                    _ => future::ready(Some(*acc)),
                }
            })
        }

        let bignum = 1_000_000_000_000_000_000;
        let example_stream = get_stream(bignum, bignum + 10 * 60 * 200)
            .map(|x| format!("{s}, {s}, {s}, {s}\n", s = x.to_string())); // 10 minute long stream (9.5MB)

        let result = upload_stream(
            S3Client::new(TEST_REGION),
            String::from(TEST_BUCKET),
            String::from("big_file.txt"),
            example_stream,
            5 * 1024 * 1024,
        )
        .await;

        if let Err(error) = result {
            panic!("{}", error);
        }
    }
}

use tonic::{transport::Server, Request, Response, Status};

use async_stream::stream;
use blake3::Hasher;
use futures_util::StreamExt;
use hexagonal::file_library_client::FileLibraryClient;
use hexagonal::{Ack, UploadFileRequest};
use std::io::Read;
use tokio::sync::mpsc;

pub mod hexagonal {
    tonic::include_proto!("hexagonal");
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileLibraryClient::connect("http://127.0.0.1:10000").await?;
    let mut stdin = std::io::stdin();

        let mut buffer = [0u8; 1024];
        let outbound = stream! {
            loop {
                let bytes_read: usize = stdin.read(&mut buffer).unwrap();
                if (bytes_read > 0) {
                    yield UploadFileRequest{
                        chunk: (&buffer[..bytes_read]).to_vec()
                    };
                } else {
                    break;
                }
            }
        };

        let response = client.upload_file(Request::new(outbound)).await?;
        println!("the hash is {}", response.into_inner().hash);

    Ok(())
}

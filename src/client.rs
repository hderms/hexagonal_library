use tonic::Request;

use async_stream::stream;
use hexagonal::file_library_client::FileLibraryClient;
use hexagonal::UploadFileRequest;
use std::io::Read;

use log::{info, trace};

pub mod hexagonal {
    tonic::include_proto!("hexagonal");
}
const ADDRESS: &str = "http://127.0.0.1:10000";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Connecting to {}", ADDRESS);
    let mut client = FileLibraryClient::connect(ADDRESS).await?;

    trace!("Opening STDIN");
    let mut stdin = std::io::stdin();

    let mut buffer = [0u8; 1024];
    let outbound = stream! {
        loop {
            let bytes_read: usize = stdin.read(&mut buffer).unwrap();
            trace!("Read {} bytes from STDIN", bytes_read);
            if (bytes_read > 0) {
                yield UploadFileRequest{
                    chunk: (&buffer[..bytes_read]).to_vec()
                };
            } else {
            trace!("End of STDIN");
                break;
            }
        }
    };

    let response = client.upload_file(Request::new(outbound)).await?;
    info!(
        "the hash returned from the server is {}",
        response.into_inner().hash
    );

    Ok(())
}

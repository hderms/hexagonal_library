use tonic::Request;

use async_stream::stream;
use hexagonal::file_library_client::FileLibraryClient;
use hexagonal::UploadFileRequest;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{io::Read, time::Instant};

use log::{debug, info, trace};

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

    let mut buffer = [0u8; 32000];
    let start = Instant::now();
    let file_size = Arc::new(AtomicUsize::new(1));
    let counter = file_size.clone();
    let outbound = stream! {
        loop {
            let bytes_read: usize = stdin.read(&mut buffer).unwrap();
            file_size.clone().fetch_add(bytes_read, Ordering::SeqCst );
            debug!("Read {} bytes from STDIN", bytes_read);
            if (bytes_read > 0) {
                yield UploadFileRequest{
                    chunk: (&buffer[..bytes_read]).to_vec()
                };
            } else {
                debug!("End of STDIN");
                break;
            }
        }
    };

    let response = client.upload_file(Request::new(outbound)).await?;
    let duration = start.elapsed();
    info!(
        "the hash returned from the server is {}",
        response.into_inner().hash
    );
    info!("took {} time", duration.as_millis());
    let duration_secs = duration.as_millis();
    let throughput = Arc::try_unwrap(counter).unwrap().into_inner() as u128 / duration_secs;
    info!("throughput: {} B/sec ", throughput);

    Ok(())
}

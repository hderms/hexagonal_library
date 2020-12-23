use tonic::Request;

use async_stream::stream;
use hexagonal::file_library_client::FileLibraryClient;
use hexagonal::{GetFileRequest, UploadFileRequest};
use std::sync::Arc;
use std::{
    io,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::{io::Read, io::Write, time::Instant};

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
    let start_upload = Instant::now();
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
    let duration = start_upload.elapsed();
    let hash = response.into_inner().hash;
    info!("the hash returned from the server is {}", hash);
    info!("upload took {} ms", duration.as_millis());
    let duration_secs = duration.as_millis();
    let bytes = Arc::try_unwrap(counter).unwrap().into_inner();
    let throughput = bytes as u128 / duration_secs;
    info!("throughput: {} B/sec ", throughput);

    info!("now getting that same file back:");
    let start_download = Instant::now();
    let mut file_stream = client
        .get_file(Request::new(GetFileRequest { hash: hash }))
        .await?
        .into_inner();

    info!("started stream");
    let mut stdout = io::stdout();
    while let Some(chunk) = file_stream.message().await? {
        stdout.write(&chunk.chunk)?;
    }

    let duration = start_download.elapsed();
    info!("download took {} ms", duration.as_millis());

    let throughput = bytes as u128 / duration.as_millis();
    info!("throughput: {} B/sec ", throughput);

    Ok(())
}

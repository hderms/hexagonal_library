use tonic::{transport::Server, Request, Response, Status};

use blake3::Hasher;
use futures_util::StreamExt;
use hexagonal::file_library_server::{FileLibrary, FileLibraryServer};
use hexagonal::{Ack, GetFileChunk, GetFileRequest, UploadFileRequest};
use std::{fs, io::Read, io::Write};
use tempfile::NamedTempFile;
use tokio::sync::mpsc;
mod directory;
use directory::{create_all_directories, get_directory_from_hash, get_file_path_from_hash};

use log::{debug, info};
pub mod hexagonal {
    tonic::include_proto!("hexagonal");
}

#[derive(Debug, Default)]
pub struct FileLibraryS {}

const UPLOAD_CHANNEL_BUFFER_SIZE: usize = 4;
const READ_BUFFER_SIZE: usize = 4096;
const STORAGE_PATH: &str = "./tmp";
#[tonic::async_trait]
impl FileLibrary for FileLibraryS {
    type GetFileStream = mpsc::Receiver<Result<GetFileChunk, Status>>;

    async fn get_file(
        &self,
        request: Request<GetFileRequest>,
    ) -> Result<Response<Self::GetFileStream>, Status> {
        debug!("Got a request: {:?}", request);
        let hash = request.into_inner().hash;
        let file_path = directory::get_file_path(STORAGE_PATH, hash);
        let mut file = fs::File::open(file_path)?;
        let (mut tx, rx) = mpsc::channel(UPLOAD_CHANNEL_BUFFER_SIZE);
        debug!(
            "opened channel with buffer size {}",
            UPLOAD_CHANNEL_BUFFER_SIZE
        );
        let mut buffer = [0u8; READ_BUFFER_SIZE];

        tokio::spawn(async move {
            loop {
                let bytes_read = file.read(&mut buffer).unwrap();
                debug!("looping through reading of file, bytes read {}", bytes_read);
                if (bytes_read > 0) {
                    let vectored_read = (&buffer[..bytes_read]).to_vec();
                    tx.send(Ok(GetFileChunk { chunk: vectored_read })).await.unwrap();
                } else {
                    info!("File stream out of bytes.");
                    break;
                }
            }
        });
        Ok(Response::new(rx))
    }

    async fn upload_file(
        &self,
        request: Request<tonic::Streaming<UploadFileRequest>>,
    ) -> Result<Response<Ack>, Status> {
        println!("Got a request: {:?}", request);
        let mut stream: tonic::Streaming<UploadFileRequest> = request.into_inner();

        let mut hasher = Hasher::new();
        let mut temp_file = NamedTempFile::new().unwrap();
        while let Some(chunk) = stream.next().await {
            let c: UploadFileRequest = chunk?;
            hasher.update(&c.chunk);
            temp_file.write_all(&c.chunk).unwrap();
        }

        temp_file.flush().unwrap();
        let hash = hasher.finalize();

        let file_path = get_file_path_from_hash(STORAGE_PATH, hash);

        //Write the file to the intended location by renaming
        fs::rename(temp_file.path(), file_path).unwrap();

        let reply = hexagonal::Ack {
            ok: true,
            hash: hasher.finalize().to_hex().to_string(),
        };

        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "127.0.0.1:10000".parse()?;
    let greeter = FileLibraryS::default();
    create_all_directories(STORAGE_PATH);

    Server::builder()
        .add_service(FileLibraryServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

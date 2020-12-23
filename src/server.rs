use tonic::{transport::Server, Request, Response, Status};

use blake3::Hasher;
use futures_util::StreamExt;
use hexagonal::file_library_server::{FileLibrary, FileLibraryServer};
use hexagonal::{Ack, GetImageChunk, GetImageRequest, UploadFileRequest};
use std::{io::Write, thread::sleep};
use tempfile::NamedTempFile;
use tokio::sync::mpsc;
mod directory;
use directory::get_directory_from_hash;

pub mod hexagonal {
    tonic::include_proto!("hexagonal");
}

#[derive(Debug, Default)]
pub struct FileLibraryS {}

#[tonic::async_trait]
impl FileLibrary for FileLibraryS {
    type GetFileStream = mpsc::Receiver<Result<GetImageChunk, Status>>;

    async fn get_file(
        &self,
        request: Request<GetImageRequest>,
    ) -> Result<Response<Self::GetFileStream>, Status> {
        println!("Got a request: {:?}", request);

        unimplemented!();
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
            temp_file.write(&c.chunk).unwrap();
        }

        temp_file.flush().unwrap();
        let hash = hasher.finalize();

        let directory_name = get_directory_from_hash(hash);

        let prefix = Path::new("./tmp");
        let directory_path = prefix.join(directory_name);
        fs::create_dir(directory_path.clone()).unwrap();

        let mut file_path = directory_path.clone();
        file_path.push(hash.to_hex().to_string());

        //rename the temporary file

        fs::rename(temp_file.path(), file_path.clone()).unwrap();

        let reply = hexagonal::Ack {
            ok: true,
            hash: hasher.finalize().to_hex().to_string(),
        };

        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:10000".parse()?;
    let greeter = FileLibraryS::default();

    Server::builder()
        .add_service(FileLibraryServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

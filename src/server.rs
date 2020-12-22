use tonic::{transport::Server, Request, Response, Status};

use blake3::Hasher;
use futures_util::StreamExt;
use hexagonal::file_library_server::{FileLibrary, FileLibraryServer};
use hexagonal::{Ack, GetImageChunk, GetImageRequest, UploadFileRequest};
use tokio::sync::mpsc;

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
        while let Some(chunk) = stream.next().await {
            let c: UploadFileRequest = chunk?;
            hasher.update(&c.chunk);
        }

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

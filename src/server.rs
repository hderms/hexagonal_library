use tonic::{transport::Server, Request, Response, Status};

use hexagonal::file_library_server::{FileLibrary, FileLibraryServer};
use hexagonal::{Ack, UploadFileRequest, GetImageChunk, GetImageRequest};
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
        request: Request<GetImageRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::GetFileStream>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        // let reply = hello_world::HelloReply {
        //     message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        // };

        // Ok(Response::new(reply)) // Send back our formatted greeting
        unimplemented!();
        
    }

    async fn upload_file(
        &self,
        request: Request<tonic::Streaming<UploadFileRequest>>,
    ) -> Result<Response<Ack>, Status> { 
        println!("Got a request: {:?}", request);

        // let reply = hello_world::HelloReply {
        //     message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        // };

        // Ok(Response::new(reply)) // Send back our formatted greeting
        unimplemented!();
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = FileLibraryS::default();

    Server::builder()
        .add_service(FileLibraryServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
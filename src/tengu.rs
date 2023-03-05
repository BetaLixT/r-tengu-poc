use prost_types::Timestamp;
use tonic::{transport::Server, Request, Response, Status};

use tengu::files_server::{Files, FilesServer};
use tengu::{
    CreateFileCommand, DeleteFilesCommand, FileEvent, FileEvents, PublicCdnEndpointQuery,
    RegisterProcessedCommand, StringResponse, ValidateFilesCommand,
};

pub mod tengu {
    tonic::include_proto!("tengu"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct FilesHandler {}

#[tonic::async_trait]
impl Files for FilesHandler {
    async fn create(
        &self,
        request: Request<CreateFileCommand>,
    ) -> Result<Response<FileEvent>, Status> {
        println!("Got a request: {:?}", request);

        let reply = tengu::FileEvent {
            id: String::from("123"),
            saga_id: Option::None,
            version: 0,
            stream: String::from("files"),
            stream_id: String::from("123456789"),
            event: String::from("created"),
            data: Option::None,
            trace_id: String::from("123456789"),
            request_id: String::from("123456789"),
            event_time: Option::None,
        };

        Ok(Response::new(reply))
    }
    async fn validate(
        &self,
        request: Request<ValidateFilesCommand>,
    ) -> Result<Response<FileEvents>, Status> {
        println!("Got a request: {:?}", request);

        let reply = tengu::FileEvents {
            file_events: vec![tengu::FileEvent {
                id: String::from("123"),
                saga_id: Option::None,
                version: 0,
                stream: String::from("files"),
                stream_id: String::from("123456789"),
                event: String::from("created"),
                data: Option::None,
                trace_id: String::from("123456789"),
                request_id: String::from("123456789"),
                event_time: Option::None,
            }],
        };
        Ok(Response::new(reply))
    }
    async fn delete(
        &self,
        request: Request<DeleteFilesCommand>,
    ) -> Result<Response<FileEvents>, Status> {
        println!("Got a request: {:?}", request);

        let reply = tengu::FileEvents {
            file_events: vec![tengu::FileEvent {
                id: String::from("123"),
                saga_id: Option::None,
                version: 0,
                stream: String::from("files"),
                stream_id: String::from("123456789"),
                event: String::from("created"),
                data: Option::None,
                trace_id: String::from("123456789"),
                request_id: String::from("123456789"),
                event_time: Option::None,
            }],
        };

        Ok(Response::new(reply))
    }
    async fn register_processed(
        &self,
        request: Request<RegisterProcessedCommand>,
    ) -> Result<Response<FileEvent>, Status> {
        println!("Got a request: {:?}", request);

        let reply = tengu::FileEvent {
            id: String::from("123"),
            saga_id: Option::None,
            version: 0,
            stream: String::from("files"),
            stream_id: String::from("123456789"),
            event: String::from("created"),
            data: Option::None,
            trace_id: String::from("123456789"),
            request_id: String::from("123456789"),
            event_time: Option::None,
        };
        Ok(Response::new(reply))
    }
    async fn list_invalid_older_than(
        &self,
        request: tonic::Request<tengu::ListInvalidFilesOlderThanQuery>,
    ) -> Result<tonic::Response<tengu::StringListResponse>, tonic::Status> {
        let reply = tengu::StringListResponse {
            items: vec![String::from("test")],
        };
        Ok(Response::new(reply))
    }

    async fn get_public_cdn_endpoint(
        &self,
        request: tonic::Request<tengu::PublicCdnEndpointQuery>,
    ) -> Result<tonic::Response<tengu::StringResponse>, tonic::Status> {
        let reply = tengu::StringResponse {
            item: String::from("test"),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = FilesHandler::default();

    Server::builder()
        .add_service(FilesServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

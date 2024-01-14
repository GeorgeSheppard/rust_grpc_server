use tonic::{transport::Server, Request, Response, Status};

use homeservice::home_server::{Home, HomeServer};
use homeservice::{TestRequest, TestResponse};

pub mod homeservice {
    tonic::include_proto!("homeservice");
}

#[derive(Debug, Default)]
pub struct MyHome {}

#[tonic::async_trait]
impl Home for MyHome {
    async fn test(
        &self,
        request: Request<TestRequest>,
    ) -> Result<Response<TestResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = TestResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let home = MyHome::default();

    Server::builder()
        .add_service(HomeServer::new(home))
        .serve(addr)
        .await?;

    Ok(())
}
use homeservice::home_client::HomeClient;
use homeservice::TestRequest;

pub mod homeservice {
    tonic::include_proto!("homeservice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HomeClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TestRequest {
        name: "Tonic".into(),
    });

    let response = client.test(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
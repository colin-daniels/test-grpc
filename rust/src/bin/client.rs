use test_grpc::pb::{echo_client::EchoClient, EchoRequest};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // http://
    // let endpoints = ["http://[::1]:50051", "http://[::1]:50052"]
    let endpoints = ["http://localhost:5143"]
        .iter()
        .map(|a| Channel::from_static(a));

    let channel = Channel::balance_list(endpoints);

    let mut client = EchoClient::new(channel);

    for i in 0..12usize {
        let request = tonic::Request::new(EchoRequest {
            message: format!("hello-{}", i).into(),
        });

        let response = client.unary_echo(request).await?;
        println!("RESPONSE={:?}", response);
    }

    Ok(())
}

use test_grpc::proto::{echo_client::EchoClient, EchoRequest};
use tonic::{transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();

    let endpoints = ["http://[::1]:50051"]
        .iter()
        .map(|a| Channel::from_static(a));

    let channel = Channel::balance_list(endpoints);
    let mut client = EchoClient::new(channel);

    for i in 0..12usize {
        let request = Request::new(EchoRequest {
            message: format!("hello-{}", i),
        });
        log::info!("request: {:?}", request);

        let response = client.unary_echo(request).await?;
        log::info!("response: {:?}", response);
    }

    Ok(())
}

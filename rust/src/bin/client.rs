use test_grpc::proto::{echo_client::EchoClient, EchoRequest};
use tonic::{transport::Channel, Request};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();

    // Single endpoint
    let endpoint = Channel::from_static("http://[::1]:50051");
    let channel = Channel::balance_list([endpoint].into_iter());
    let mut client = EchoClient::new(channel);

    // Test server by sending 12 requests
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

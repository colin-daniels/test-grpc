use sha2::{Digest, Sha512};
use std::net::SocketAddr;
use test_grpc::proto;
use test_grpc::proto::{EchoRequest, EchoResponse};
use tonic::{transport::Server, Request, Response, Status};

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct EchoServer {
    addr: SocketAddr,
}

fn sha512(data: String) -> sha2::digest::Output<Sha512> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize()
}

#[tonic::async_trait]
impl proto::echo_server::Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let remote_addr = request.remote_addr().unwrap();
        let hash = sha512(request.into_inner().message);
        log::info!("computed sha512: {:x} (for {})", hash, remote_addr);

        let message = format!("{:x} (from {})", hash, self.addr);
        Ok(Response::new(EchoResponse { message }))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init_timed();

    let addr = "[::1]:50051".parse()?;
    let server = EchoServer { addr };

    let serve = Server::builder()
        .add_service(proto::echo_server::EchoServer::new(server))
        .serve(addr);

    tokio::spawn(async move {
        if let Err(e) = serve.await {
            log::error!("Error = {:?}", e);
        }
    })
    .await?;

    Ok::<(), anyhow::Error>(())
}

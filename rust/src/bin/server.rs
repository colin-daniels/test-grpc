use sha2::{Digest, Sha512};
use std::net::SocketAddr;
use test_grpc::pb;
use test_grpc::pb::{EchoRequest, EchoResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct EchoServer {
    addr: SocketAddr,
}

#[tonic::async_trait]
impl pb::echo_server::Echo for EchoServer {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let mut hasher = Sha512::new();
        hasher.update(request.into_inner().message);

        let message = format!("{:x} (from {})", hasher.finalize(), self.addr);
        Ok(Response::new(EchoResponse { message }))
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() {
    // let addrs = ["[::1]:50051", "[::1]:50052"];

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let addrs = ["[::1]:50051"];

        let (tx, mut rx) = mpsc::unbounded_channel();

        for addr in &addrs {
            let addr = addr.parse()?;
            let tx = tx.clone();

            let server = EchoServer { addr };
            let serve = Server::builder()
                .add_service(pb::echo_server::EchoServer::new(server))
                .serve(addr);

            tokio::spawn(async move {
                if let Err(e) = serve.await {
                    eprintln!("Error = {:?}", e);
                }

                tx.send(()).unwrap();
            });
        }

        rx.recv().await;

        Ok::<(), Box<dyn std::error::Error>>(())
    })
    .unwrap();
}

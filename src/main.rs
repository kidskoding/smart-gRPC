pub mod my_sentinel;

use tonic::transport::Server;

use crate::my_sentinel::{MySentinel, sentinel::smart_sentinel_server::SmartSentinelServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let sentinel = MySentinel::default();

    Server::builder()
        .add_service(SmartSentinelServer::new(sentinel))
        .serve(addr)
        .await?;

    Ok(())
}
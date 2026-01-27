pub mod my_sentinel;

use dotenvy::dotenv;
use tonic::transport::Server;

use crate::my_sentinel::{MySentinel, sentinel::smart_sentinel_server::SmartSentinelServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let ai_client = rig_vertexai::Client::from_env();

    let addr = "[::1]:50051".parse()?;
    let sentinel = MySentinel { ai_client };

    Server::builder()
        .add_service(SmartSentinelServer::new(sentinel))
        .serve(addr)
        .await?;

    Ok(())
}
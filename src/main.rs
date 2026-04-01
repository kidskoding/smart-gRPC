use dotenvy::dotenv;
use rig::client::ProviderClient;
use rig::providers::openai;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use smart_gRPC::my_sentinel::MySentinel;
use smart_gRPC::proto::smart_sentinel_server::SmartSentinelServer;
use smart_gRPC::proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let openai_client = openai::Client::from_env();

    let addr = "[::1]:8000".parse()?;
    let sentinel = MySentinel { openai_client };

    println!("gRPC server starting at [::1]:8000!");

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(SmartSentinelServer::new(sentinel))
        .serve(addr)
        .await?;

    Ok(())
}

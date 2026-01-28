use qdrant_client::Qdrant;
use qdrant_client::qdrant::QueryPointsBuilder;
use rig::prelude::*;
use rig::embeddings::EmbeddingModel;
use rig::providers::azure::TEXT_EMBEDDING_3_SMALL;
use rig::providers::openai;
use rig_qdrant::QdrantVectorStore;

pub async fn init_memory() -> QdrantVectorStore<impl EmbeddingModel> {
    let openai_client = openai::Client::from_env();   
    let embedding_model = openai_client.embedding_model(TEXT_EMBEDDING_3_SMALL);

    let qdrant_client = Qdrant::from_url("http://localhost:6334")
        .build()
        .expect("failed to build Qdrant client");

    let query_params = QueryPointsBuilder::new("banking_policy_collection")
        .with_payload(true)
        .build();

    QdrantVectorStore::new(qdrant_client, embedding_model, query_params)
}

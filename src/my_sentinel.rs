use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::embeddings::EmbeddingModel;
use rig::providers::openai::{self, GPT_4O, EmbeddingModel as OpenAIEmbeddingModel};
use rig::vector_store::VectorStoreIndex;

use crate::proto::smart_sentinel_server::SmartSentinel;
use crate::proto::{TransactionRequest, AgentDecision};

use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct MySentinel<M: EmbeddingModel + Clone + Send + Sync + 'static> {
    pub openai_client: openai::Client,
    pub policy_index: VectorStoreIndex<M>,
}

#[tonic::async_trait]
impl<M> SmartSentinel for MySentinel<M> {
    async fn inspect(&self, request: Request<TransactionRequest>) -> Result<Response<AgentDecision>, tonic::Status> {
        let tx = request.into_inner();

        let agent = self.openai_client
            .agent(GPT_4O)
            .preamble("You are a banking security sentinel. 
                Policy: Transactions over $1000 are denied for new users.
                Policy: Deny transactions for gambling or crypto merchants.")
            .dynamic_context(2, self.policy_index.clone())
            .max_tokens(1024)
            .build();

        let prompt_text = format!(
            "Analyze this transaction: UserID: {}, Amount: {}, Merchant: {}. 
            Reply with 'APPROVE' or 'DENY' and your reasoning.", 
            tx.user_id, tx.amount, tx.merchant
        );

        let ai_response = agent.prompt(prompt_text).await
            .map_err(|e| Status::internal(format!("Vertex AI Error: {}", e)))?;

        Ok(Response::new(AgentDecision {
            allowed: ai_response.to_uppercase().contains("APPROVE"),
            reasoning: ai_response,
        }))
    }
}

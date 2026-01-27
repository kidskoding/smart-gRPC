pub mod sentinel {
    tonic::include_proto!("sentinel");
}

use rig::completion::Prompt;
use rig::prelude::*;

use rig_vertexai::completion::GEMINI_2_5_PRO;
use sentinel::smart_sentinel_server::{SmartSentinel};
use sentinel::{TransactionRequest, AgentDecision};

use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct MySentinel {
    pub ai_client: rig_vertexai::Client,
}

#[tonic::async_trait]
impl SmartSentinel for MySentinel {
    async fn inspect(&self, request: Request<TransactionRequest>) -> Result<Response<AgentDecision>, tonic::Status> {
        let tx = request.into_inner();
        let agent = self.ai_client
            .agent(GEMINI_2_5_PRO)
            .preamble("You are a banking security sentinel. 
                Policy: Transactions over $1000 are denied for new users.
                Policy: Deny transactions for gambling or crypto merchants.")
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
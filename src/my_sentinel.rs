use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::openai::{self, GPT_4O};

use crate::proto::smart_sentinel_server::SmartSentinel;
use crate::proto::{AgentDecision, TransactionRequest};

use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct MySentinel {
    pub openai_client: openai::Client,
}

#[tonic::async_trait]
impl SmartSentinel for MySentinel {
    async fn inspect(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<AgentDecision>, Status> {
        let tx = request.into_inner();

        let agent = self.openai_client
            .agent(GPT_4O)
            .preamble(
                "You are a banking security sentinel. \
                 Policy: Transactions over $1000 are denied for new users (account age < 30 days). \
                 Policy: Deny transactions for gambling or crypto merchants. \
                 Policy: Flag transactions from unusual locations. \
                 Respond with APPROVE or DENY followed by your reasoning.",
            )
            .max_tokens(1024)
            .build();

        let prompt_text = format!(
            "Analyze this transaction: UserID: {}, Amount: ${}, Merchant: {}, \
             Currency: {}, Location: {}, Account Age: {} days.",
            tx.user_id, tx.amount, tx.merchant, tx.currency, tx.location, tx.user_account_age
        );

        let ai_response: String = agent
            .prompt(prompt_text)
            .await
            .map_err(|e| Status::internal(format!("OpenAI Error: {}", e)))?;

        Ok(Response::new(AgentDecision {
            allowed: ai_response.to_uppercase().contains("APPROVE"),
            reasoning: ai_response,
        }))
    }
}

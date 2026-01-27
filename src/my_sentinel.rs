pub mod sentinel {
    tonic::include_proto!("sentinel");
}

use sentinel::smart_sentinel_server::{SmartSentinel};
use sentinel::{TransactionRequest, AgentDecision};
use tonic::{Request, Response};

#[derive(Default)]
pub struct MySentinel {}

#[tonic::async_trait]
impl SmartSentinel for MySentinel {
    async fn inspect(&self, request: Request<TransactionRequest>) -> Result<Response<AgentDecision>, tonic::Status> {
        let tx = request.into_inner();
        
        Ok(Response::new(AgentDecision {
            allowed: true,
            reasoning: format!("verified transaction for user: {}", tx.user_id),
        }))
    }
}
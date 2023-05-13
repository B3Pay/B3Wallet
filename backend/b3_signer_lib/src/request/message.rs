use ic_cdk::api::time as ic_timestamp;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize)]
pub struct SignMessageRequest {
    pub id: String,
    pub message: Vec<u8>,
    pub deadline: u64,
}

impl SignMessageRequest {
    pub fn new(message: Vec<u8>, deadline: Option<u64>) -> Self {
        let now = ic_timestamp();
        // now in nanoseconds since the epoch (1970-01-01)
        // and default deadline is 15 minutes
        let deadline = deadline.unwrap_or(now + 15 * 60 * 1_000_000_000);

        SignMessageRequest {
            id: "".to_string(),
            message,
            deadline,
        }
    }
}

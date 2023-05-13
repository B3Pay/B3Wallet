use b3_helper::types::CanisterId;
use ic_cdk::api::time as ic_timestamp;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize)]
pub struct SendIcpRequest {
    pub id: String,
    pub to: CanisterId,
    pub amount: u64,
    pub deadline: u64,
}

impl SendIcpRequest {
    pub fn new(amount: u64, to: CanisterId, deadline: Option<u64>) -> Self {
        let now = ic_timestamp();
        // now in nanoseconds since the epoch (1970-01-01)
        // and default deadline is 15 minutes
        let deadline = deadline.unwrap_or(now + 15 * 60 * 1_000_000_000);

        SendIcpRequest {
            id: "".to_string(),
            to,
            amount,
            deadline,
        }
    }
}

use b3_utils::types::OperationId;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct RequestNonce(pub OperationId);

impl RequestNonce {
    pub fn new() -> Self {
        RequestNonce(0)
    }

    pub fn request(&self) -> OperationId {
        self.0
    }

    pub fn increment_request(&mut self) -> OperationId {
        self.0 += 1;

        self.0
    }

    /// increment the request counter and return the new value
    pub fn generate_next_request_id(&mut self) -> OperationId {
        self.0 += 1;

        self.0
    }
}

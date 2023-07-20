use b3_utils::types::RequestId;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct RequestNonce(pub RequestId);

impl RequestNonce {
    pub fn new() -> Self {
        RequestNonce(0)
    }

    pub fn request(&self) -> RequestId {
        self.0
    }

    pub fn increment_request(&mut self) -> RequestId {
        self.0 += 1;

        self.0
    }

    /// increment the request counter and return the new value
    pub fn generate_next_request_id(&mut self) -> RequestId {
        self.0 += 1;

        self.0
    }
}

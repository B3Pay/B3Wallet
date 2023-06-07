use b3_helper_lib::types::RequestId;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct RequestCounters {
    request: RequestId,
}

impl RequestCounters {
    pub fn new() -> Self {
        RequestCounters { request: 0 }
    }

    pub fn request(&self) -> RequestId {
        self.request
    }

    pub fn increment_request(&mut self) -> RequestId {
        self.request += 1;

        self.request
    }

    /// increment the request counter and return the new value
    pub fn generate_next_request_id(&mut self) -> RequestId {
        self.request += 1;

        self.request
    }
}

#[cfg(test)]
use crate::mocks::ic_timestamp;

#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::types::Status;

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct SignedTransaction {
    pub data: Vec<u8>,
    pub status: Status,
    pub timestamp: u64,
}

impl SignedTransaction {
    pub fn new(data: Vec<u8>) -> Self {
        SignedTransaction {
            data,
            status: Status::Pending,
            timestamp: ic_timestamp(),
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    pub fn is_pending(&self) -> bool {
        self.status == Status::Pending
    }
}

pub mod state;

use crate::{error::WalletError, request::PendingRequest, types::SignedMessage};
use b3_helper::error::TrapError;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, PartialEq, Clone, Deserialize)]
pub enum RequestStatus {
    Pending,
    Success,
    Fail,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ConfirmedRequest {
    error: String,
    timestamp: u64,
    message: SignedMessage,
    status: RequestStatus,
    request: PendingRequest,
}

impl From<ConfirmedRequest> for PendingRequest {
    fn from(request: ConfirmedRequest) -> Self {
        request.request
    }
}

impl From<PendingRequest> for ConfirmedRequest {
    fn from(request: PendingRequest) -> Self {
        ConfirmedRequest {
            error: "".to_owned(),
            timestamp: 0,
            message: SignedMessage::default(),
            status: RequestStatus::Pending,
            request,
        }
    }
}

impl ConfirmedRequest {
    pub fn new(request: &PendingRequest) -> Self {
        ConfirmedRequest {
            error: "".to_owned(),
            timestamp: 0,
            request: request.clone(),
            status: RequestStatus::Pending,
            message: SignedMessage::default(),
        }
    }

    pub fn confirm(&mut self, message: SignedMessage) -> Self {
        self.status = RequestStatus::Success;
        self.timestamp = ic_cdk::api::time();
        self.message = message;

        self.clone()
    }

    pub fn reject(&mut self, error: WalletError) -> Self {
        self.status = RequestStatus::Fail;
        self.error = error.to_string();
        self.timestamp = ic_cdk::api::time();

        self.clone()
    }

    pub fn is_successful(&self) -> bool {
        self.status == RequestStatus::Success
    }

    pub fn is_failed(&self) -> bool {
        self.status == RequestStatus::Fail
    }

    pub fn is_pending(&self) -> bool {
        self.status == RequestStatus::Pending
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

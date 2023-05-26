pub mod state;

use crate::{error::RequestError, pending::PendingRequest, types::ConsentMessageResponse};
use b3_helper_lib::error::TrapError;
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
    message: ConsentMessageResponse,
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
            message: ConsentMessageResponse::default(),
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
            message: ConsentMessageResponse::default(),
        }
    }

    pub fn confirm(&mut self, message: ConsentMessageResponse) -> Self {
        self.status = RequestStatus::Success;
        self.timestamp = ic_cdk::api::time();
        self.message = message;

        self.clone()
    }

    pub fn reject(&mut self, error: RequestError) -> Self {
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

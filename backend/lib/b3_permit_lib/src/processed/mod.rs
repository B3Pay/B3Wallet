pub mod state;

#[cfg(test)]
use crate::mocks::ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

use crate::{error::RequestError, pending::PendingRequest, types::ConsentMessageResponse};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, PartialEq, Clone, Deserialize)]
pub enum RequestStatus {
    Pending,
    Success,
    Fail,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ProcessedRequest {
    message: ConsentMessageResponse,
    error: Option<RequestError>,
    method: String,
    request: PendingRequest,
    status: RequestStatus,
    timestamp: u64,
}

impl From<ProcessedRequest> for PendingRequest {
    fn from(request: ProcessedRequest) -> Self {
        request.request
    }
}

impl From<PendingRequest> for ProcessedRequest {
    fn from(request: PendingRequest) -> Self {
        let error = request.get_error();

        let status = if error.is_some() {
            RequestStatus::Fail
        } else {
            RequestStatus::Success
        };

        let message = if let Some(error) = &error {
            ConsentMessageResponse::from(error)
        } else {
            ConsentMessageResponse::default()
        };

        ProcessedRequest {
            error,
            timestamp: ic_timestamp(),
            method: request.method(),
            message,
            status,
            request,
        }
    }
}

impl ProcessedRequest {
    pub fn new(request: &PendingRequest) -> Self {
        ProcessedRequest {
            error: None,
            timestamp: ic_timestamp(),
            method: request.method(),
            request: request.clone(),
            status: RequestStatus::Pending,
            message: ConsentMessageResponse::default(),
        }
    }

    pub fn succeed(&mut self, message: ConsentMessageResponse) -> Self {
        self.status = RequestStatus::Success;
        self.timestamp = ic_timestamp();
        self.message = message;

        self.clone()
    }

    pub fn fail(&mut self, error: RequestError) -> Self {
        self.status = RequestStatus::Fail;
        self.message = ConsentMessageResponse::from(&error);
        self.error = Some(error);
        self.timestamp = ic_timestamp();

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

    pub fn get_error(&self) -> &Option<RequestError> {
        &self.error
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

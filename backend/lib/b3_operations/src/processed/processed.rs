#[cfg(test)]
use b3_utils::mocks::time_mock as ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

use crate::{error::OperationError, operation::result::OperationResult, pending::PendingRequest};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum RequestStatus {
    Expired,
    Pending,
    Success,
    Fail,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProcessedRequest {
    timestamp: u64,
    method: String,
    error: Option<String>,
    status: RequestStatus,
    request: PendingRequest,
    result: Option<OperationResult>,
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

        ProcessedRequest {
            error,
            timestamp: ic_timestamp(),
            method: request.method(),
            result: None,
            status,
            request,
        }
    }
}

impl ProcessedRequest {
    pub fn new(request: &PendingRequest) -> Self {
        ProcessedRequest {
            error: None,
            result: None,
            timestamp: ic_timestamp(),
            method: request.method(),
            request: request.clone(),
            status: RequestStatus::Pending,
        }
    }

    pub fn succeed(&mut self, message: OperationResult) -> Self {
        self.status = RequestStatus::Success;
        self.request.status = RequestStatus::Success;
        self.timestamp = ic_timestamp();
        self.result = Some(message);

        self.clone()
    }

    pub fn fail(&mut self, error: OperationError) -> Self {
        self.status = RequestStatus::Fail;
        self.request.status = RequestStatus::Fail;
        self.error = Some(error.to_string());
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

    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

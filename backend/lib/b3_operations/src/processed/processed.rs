#[cfg(test)]
use b3_utils::mocks::time_mock as ic_timestamp;
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;

use crate::{error::OperationError, operation::result::OperationResult, pending::PendingOperation};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum OperationStatus {
    Expired,
    Pending,
    Success,
    Fail,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProcessedOperation {
    timestamp: u64,
    method: String,
    error: Option<String>,
    status: OperationStatus,
    request: PendingOperation,
    result: Option<OperationResult>,
}

impl From<ProcessedOperation> for PendingOperation {
    fn from(request: ProcessedOperation) -> Self {
        request.request
    }
}

impl From<PendingOperation> for ProcessedOperation {
    fn from(request: PendingOperation) -> Self {
        let error = request.get_error();

        let status = if error.is_some() {
            OperationStatus::Fail
        } else {
            OperationStatus::Success
        };

        ProcessedOperation {
            error,
            timestamp: ic_timestamp(),
            method: request.method(),
            result: None,
            status,
            request,
        }
    }
}

impl ProcessedOperation {
    pub fn new(request: &PendingOperation) -> Self {
        ProcessedOperation {
            error: None,
            result: None,
            timestamp: ic_timestamp(),
            method: request.method(),
            request: request.clone(),
            status: OperationStatus::Pending,
        }
    }

    pub fn succeed(&mut self, message: OperationResult) -> Self {
        self.status = OperationStatus::Success;
        self.request.status = OperationStatus::Success;
        self.timestamp = ic_timestamp();
        self.result = Some(message);

        self.clone()
    }

    pub fn fail(&mut self, error: OperationError) -> Self {
        self.status = OperationStatus::Fail;
        self.request.status = OperationStatus::Fail;
        self.error = Some(error.to_string());
        self.timestamp = ic_timestamp();

        self.clone()
    }

    pub fn is_successful(&self) -> bool {
        self.status == OperationStatus::Success
    }

    pub fn is_failed(&self) -> bool {
        self.status == OperationStatus::Fail
    }

    pub fn is_pending(&self) -> bool {
        self.status == OperationStatus::Pending
    }

    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

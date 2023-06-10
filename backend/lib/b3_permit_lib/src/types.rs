use crate::{
    error::RequestError,
    pending::PendingRequest,
    processed::ProcessedRequest,
    request::{Request, RequestTrait},
    signer::Signer,
};
use b3_helper_lib::types::{RequestId, SignerId};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type SignerMap = HashMap<SignerId, Signer>;

pub type PendingRequestList = Vec<PendingRequest>;

pub type ProcessedRequestList = Vec<ProcessedRequest>;

pub type Responses = BTreeMap<SignerId, RequestResponse>;

pub type PendingRequestMap = BTreeMap<RequestId, PendingRequest>;

pub type ProcessedRequestMap = BTreeMap<RequestId, ProcessedRequest>;

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum RequestResponse {
    Confirm,
    Reject,
}

impl RequestResponse {
    pub fn is_confirm(&self) -> bool {
        match self {
            RequestResponse::Confirm => true,
            _ => false,
        }
    }

    pub fn is_reject(&self) -> bool {
        match self {
            RequestResponse::Reject => true,
            _ => false,
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ConsentMessage {
    pub message: String,
    pub method: String,
}

impl ConsentMessage {
    pub fn new(request: &Request) -> Self {
        let method = request.method_name();
        let message = format!("You are about to call {}", method);

        ConsentMessage {
            message,
            method: method.to_string(),
        }
    }
}

impl From<&Request> for ConsentMessage {
    fn from(request: &Request) -> Self {
        ConsentMessage::new(request)
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ErrorInfo {
    pub error_code: u64,
    pub description: String,
}

impl From<&RequestError> for ConsentMessage {
    fn from(error: &RequestError) -> Self {
        error.into()
    }
}

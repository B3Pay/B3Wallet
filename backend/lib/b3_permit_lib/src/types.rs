use crate::{
    error::RequestError,
    pending::{PendingRequest, RequestArgs},
    processed::ProcessedRequest,
    signer::Signer,
};
use b3_helper_lib::{
    error::TrapError,
    types::{RequestId, SignerId},
};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type SignerMap = HashMap<SignerId, Signer>;

pub type PendingRequestList = Vec<PendingRequest>;

pub type ProcessedRequestList = Vec<ProcessedRequest>;

pub type Response = BTreeMap<SignerId, RequestResponse>;

pub type ResponseMap = BTreeMap<RequestId, RequestResponse>;

pub type PendingRequestMap = BTreeMap<RequestId, PendingRequest>;

pub type ProcessedRequestMap = BTreeMap<RequestId, ProcessedRequest>;

#[enum_dispatch]
pub trait RequestResponseTrait {
    fn is_confirm(&self) -> bool;
    fn is_reject(&self) -> bool;
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct Confirm;

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct Reject;

impl RequestResponseTrait for Confirm {
    fn is_confirm(&self) -> bool {
        true
    }
    fn is_reject(&self) -> bool {
        false
    }
}

impl RequestResponseTrait for Reject {
    fn is_reject(&self) -> bool {
        true
    }
    fn is_confirm(&self) -> bool {
        false
    }
}

#[enum_dispatch(RequestResponseTrait)]
#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum RequestResponse {
    Confirm,
    Reject,
}

// ICRC-21: Canister Call Consent Messages --------------------------------------
#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct ConsentPreferences {
    pub language: String,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct ConsentMessageRequest {
    pub method: String,
    pub arg: RequestArgs,
    pub consent_preferences: ConsentPreferences,
}

impl From<&RequestArgs> for ConsentMessageRequest {
    fn from(request: &RequestArgs) -> Self {
        ConsentMessageRequest {
            method: request.request.to_string(),
            arg: request.clone(),
            consent_preferences: ConsentPreferences {
                language: "en-US".to_string(),
            },
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ConsendInfo {
    pub consent_message: String,
    pub language: String,
}

impl Default for ConsendInfo {
    fn default() -> Self {
        ConsendInfo {
            consent_message: "".to_string(),
            language: "en-US".to_string(),
        }
    }
}

impl From<ConsendInfo> for ConsentMessageResponse {
    fn from(consent_info: ConsendInfo) -> Self {
        ConsentMessageResponse::Valid(consent_info)
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ErrorInfo {
    pub error_code: u64,
    pub description: String,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub enum ConsentMessageResponse {
    Valid(ConsendInfo),
    Forbidden(ErrorInfo),
    MalformedCall(ErrorInfo),
    Other(String),
}

impl From<&RequestError> for ConsentMessageResponse {
    fn from(error: &RequestError) -> Self {
        match error {
            RequestError::InvalidRequest => ConsentMessageResponse::MalformedCall(ErrorInfo {
                error_code: 400,
                description: error.to_owned().to_string(),
            }),
            RequestError::SignerRoleNotAuthorized(e) => {
                ConsentMessageResponse::Forbidden(ErrorInfo {
                    error_code: 403,
                    description: e.to_string(),
                })
            }
            _ => ConsentMessageResponse::Other(error.to_owned().to_string()),
        }
    }
}

impl Default for ConsentMessageResponse {
    fn default() -> Self {
        ConsentMessageResponse::Valid(ConsendInfo::default())
    }
}

impl<T: std::fmt::Display> From<T> for ConsentMessageResponse {
    fn from(item: T) -> Self {
        ConsentMessageResponse::Other(item.to_string())
    }
}

pub trait Service {
    fn consent_message(&self, request: ConsentMessageRequest) -> Option<ConsentMessageResponse>;
}

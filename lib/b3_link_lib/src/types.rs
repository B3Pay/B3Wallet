use crate::{confirmed::ConfirmedRequest, pending::PendingRequest, signer::Signer};
use b3_helper_lib::types::SignerId;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type RequestId = usize;

pub type Deadline = u64;

pub type AccountId = String;

pub type Metadata = HashMap<String, String>;

pub type SignerMap = HashMap<SignerId, Signer>;

pub type PendingRequestMap = BTreeMap<RequestId, PendingRequest>;

pub type ConfirmedRequestMap = HashMap<RequestId, ConfirmedRequest>;

pub type PendingRequestList = Vec<PendingRequest>;

// ICRC-21: Canister Call Consent Messages --------------------------------------
#[derive(CandidType, Clone, Debug)]
pub struct ConsentPreferences {
    pub language: String,
}

#[derive(CandidType, Clone, Debug)]
pub struct ConsentMessageRequest {
    pub method: String,
    pub arg: PendingRequest,
    pub consent_preferences: ConsentPreferences,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ConsendInfo {
    pub consent_message: String,
    pub language: String,
}

impl Default for ConsendInfo {
    fn default() -> Self {
        ConsendInfo {
            consent_message: "".to_string(),
            language: "".to_string(),
        }
    }
}

impl From<ConsendInfo> for ConsentMessageResponse {
    fn from(consent_info: ConsendInfo) -> Self {
        ConsentMessageResponse::Valid(consent_info)
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct ErrorInfo {
    pub error_code: u64,
    pub description: String,
}

#[derive(CandidType, Clone, Deserialize)]
pub enum ConsentMessageResponse {
    Valid(ConsendInfo),
    Forbidden(ErrorInfo),
    MalformedCall(ErrorInfo),
    Other(String),
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

use crate::{
    error::PermitError,
    pending::new::PendingRequest,
    processed::processed::ProcessedRequest,
    request::request::{Request, RequestTrait},
    signer::signer::Signer,
};
use b3_helper_lib::types::{RequestId, SignerId};
use b3_wallet_lib::setting::WalletSettings;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type Signers = Vec<Signer>;

pub type SignerIds = Vec<SignerId>;

pub type SignerMap = HashMap<SignerId, Signer>;

pub type PendingRequestList = Vec<PendingRequest>;

pub type ProcessedRequestList = Vec<ProcessedRequest>;

pub type ResponseMap = BTreeMap<SignerId, Response>;

pub type PendingRequestMap = BTreeMap<RequestId, PendingRequest>;

pub type ProcessedRequestMap = BTreeMap<RequestId, ProcessedRequest>;

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletSettingsAndSigners {
    pub signers: SignerMap,
    pub settings: WalletSettings,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum Response {
    Confirm,
    Reject,
}

impl Response {
    pub fn is_confirm(&self) -> bool {
        match self {
            Response::Confirm => true,
            _ => false,
        }
    }

    pub fn is_reject(&self) -> bool {
        match self {
            Response::Reject => true,
            _ => false,
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ConsentMessage {
    pub message: String,
    pub reason: String,
    pub title: String,
}

impl ConsentMessage {
    pub fn new(request: &Request, reason: String) -> Self {
        let method = request.method_name();
        let message = request.to_string();

        ConsentMessage {
            message,
            reason,
            title: method.to_string(),
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ErrorInfo {
    pub error_code: u64,
    pub description: String,
}

impl From<&PermitError> for ConsentMessage {
    fn from(error: &PermitError) -> Self {
        error.into()
    }
}

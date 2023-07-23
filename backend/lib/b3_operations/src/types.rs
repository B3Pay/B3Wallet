use crate::{
    error::OperationError,
    operation::{OperationTrait, Operations},
    pending::PendingRequest,
    processed::ProcessedRequest,
    response::Response,
    signer::Signer,
};
use b3_utils::types::{RequestId, SignerId};
use b3_wallet_lib::setting::WalletSettings;
use candid::{CandidType, Deserialize};
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

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ConsentMessage {
    pub message: String,
    pub reason: String,
    pub title: String,
}

impl ConsentMessage {
    pub fn new(request: &Operations, reason: String) -> Self {
        let title = request.title();
        let message = request.message();

        ConsentMessage {
            message,
            reason,
            title,
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ErrorInfo {
    pub error_code: u64,
    pub description: String,
}

impl From<&OperationError> for ConsentMessage {
    fn from(error: &OperationError) -> Self {
        error.into()
    }
}

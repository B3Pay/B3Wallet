use crate::{
    error::RequestError,
    pending::{PendingRequest, RequestArgs, RequestTrait},
    processed::ProcessedRequest,
    signer::Signer,
};
use b3_helper_lib::{
    identifier::AccountIdentifier,
    tokens::Tokens,
    types::{CanisterId, RequestId, SignerId},
};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type SignerMap = HashMap<SignerId, Signer>;

pub type PendingRequestList = Vec<PendingRequest>;

pub type ProcessedRequestList = Vec<ProcessedRequest>;

pub type Responses = BTreeMap<SignerId, RequestResponse>;

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

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct ConsentMessageRequest {
    pub method: String,
    pub arg: RequestArgs,
}

impl From<&RequestArgs> for ConsentMessageRequest {
    fn from(request: &RequestArgs) -> Self {
        ConsentMessageRequest {
            method: request.request.method_name(),
            arg: request.clone(),
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ConsentInfo {
    pub consent_message: String,
}

impl ConsentInfo {
    pub fn new(consent_message: String) -> Self {
        ConsentInfo { consent_message }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub enum ConsentMessages {
    TopUpCanister { canister_id: String, amount: u64 },
}

impl From<ConsentMessages> for ConsentMessageResponse {
    fn from(consent_messages: ConsentMessages) -> Self {
        match consent_messages {
            ConsentMessages::TopUpCanister {
                canister_id,
                amount,
            } => {
                let consent_message = format!(
                    "You are about to top up canister {} with {} ICP. Do you want to continue?",
                    canister_id, amount
                );
                ConsentMessageResponse::Valid(ConsentInfo::new(consent_message))
            }
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub enum ConsentSuccess {
    TopUpCanister {
        canister_id: CanisterId,
        amount: u128,
    },
    IcpTransfer {
        to: AccountIdentifier,
        amount: Tokens,
        block_number: u64,
    },
}

impl From<ConsentSuccess> for ConsentMessageResponse {
    fn from(consent_success: ConsentSuccess) -> Self {
        match consent_success {
            ConsentSuccess::TopUpCanister {
                canister_id,
                amount,
            } => {
                let consent_message = format!(
                    "You have successfully topped up canister {} with {} ICP.",
                    canister_id, amount
                );
                ConsentMessageResponse::Valid(ConsentInfo::new(consent_message))
            }
            ConsentSuccess::IcpTransfer {
                to,
                amount,
                block_number,
            } => {
                let consent_message = format!(
                    "You have successfully transferred {} ICP to canister {} at block number {}.",
                    amount, to, block_number
                );
                ConsentMessageResponse::Valid(ConsentInfo::new(consent_message))
            }
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub enum ConsentError {
    TopUpCanister { canister_id: String, amount: u64 },
}

impl From<ConsentError> for ConsentMessageResponse {
    fn from(consent_error: ConsentError) -> Self {
        match consent_error {
            ConsentError::TopUpCanister {
                canister_id,
                amount,
            } => {
                let consent_message = format!(
                    "Failed to top up canister {} with {} ICP.",
                    canister_id, amount
                );
                ConsentMessageResponse::Valid(ConsentInfo::new(consent_message))
            }
        }
    }
}

impl From<ConsentInfo> for ConsentMessageResponse {
    fn from(consent_info: ConsentInfo) -> Self {
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
    Valid(ConsentInfo),
    Forbidden(ErrorInfo),
    MalformedCall(ErrorInfo),
    Other(String),
}

impl From<&RequestError> for ConsentMessageResponse {
    fn from(error: &RequestError) -> Self {
        error.into()
    }
}

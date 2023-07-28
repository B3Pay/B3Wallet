use crate::{
    error::OperationError,
    operation::{Operation, OperationTrait},
    pending::PendingOperation,
    processed::ProcessedOperation,
    response::Response,
    user::User,
};
use b3_utils::types::{OperationId, UserId};
use b3_wallet_lib::setting::WalletSettings;
use candid::{CandidType, Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type Signers = Vec<User>;

pub type UserIds = Vec<UserId>;

pub type UserMap = HashMap<UserId, User>;

pub type PendingOperations = Vec<PendingOperation>;

pub type ProcessedOperations = Vec<ProcessedOperation>;

pub type ResponseMap = BTreeMap<UserId, Response>;

pub type PendingOperationMap = BTreeMap<OperationId, PendingOperation>;

pub type ProcessedOperationMap = BTreeMap<OperationId, ProcessedOperation>;

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletSettingsAndSigners {
    pub signers: UserMap,
    pub settings: WalletSettings,
}

#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct ConsentMessage {
    pub message: String,
    pub reason: String,
    pub title: String,
}

impl ConsentMessage {
    pub fn new(request: &Operation, reason: String) -> Self {
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

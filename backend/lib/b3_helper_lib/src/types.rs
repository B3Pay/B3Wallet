use crate::{
    account_identifier::AccountIdentifier,
    error::{helper_error::HelperError, notify_error::NotifyError, transfer_error::TransferError},
    icp_token::ICPToken,
    subaccount::Subaccount,
    timestamp::NanoTimeStamp,
};
use ic_cdk::{
    api::management_canister::main::{CanisterInstallMode, CanisterStatusResponse},
    export::{
        candid::{CandidType, Encode},
        serde::{Deserialize, Serialize},
        Principal,
    },
};

use std::collections::HashMap;

pub type Metadata = HashMap<String, String>;

pub type CanisterId = Principal;
pub type SignerId = Principal;

pub type RequestId = usize;
pub type Deadline = u64;

pub type Cycles = u128;

pub type Version = String;

pub type Blob = Vec<u8>;

pub type WasmSize = usize;
pub type WasmModule = Vec<u8>;
pub type WasmHash = [u8; 32];
pub type WasmHashString = String;
pub type WasmVersion = String;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct WasmDetails {
    pub hash: WasmHash,
    pub size: WasmSize,
}

pub struct WalletCanisterInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Controller {
    pub name: String,
    pub metadata: Metadata,
}

impl Controller {
    pub fn new(name: String, metadata: Option<Metadata>) -> Self {
        Self {
            name,
            metadata: metadata.unwrap_or_default(),
        }
    }
}

pub type ControllerId = Principal;
pub type ControllerIds = Vec<ControllerId>;

pub type ControllerMap = HashMap<ControllerId, Controller>;

#[derive(CandidType, Clone, Deserialize)]
pub struct InititializeWalletArgs {
    pub controllers: ControllerMap,
    pub metadata: Option<Metadata>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterInitArgs {
    pub owner_id: SignerId,
    pub system_id: CanisterId,
}

impl WalletCanisterInitArgs {
    pub fn encode(&self) -> Result<Vec<u8>, HelperError> {
        Encode!(&self).map_err(|e| HelperError::EncodeError(e.to_string()))
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAllowanceArgs {
    pub limit: Option<u8>,
    pub metadata: Metadata,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Default, Deserialize, Serialize)]
pub enum TransactionStatus {
    #[default]
    Pending,
    Success,
    Failed,
}

#[derive(CandidType, Default, Clone, Deserialize, Serialize)]
pub struct AccountsNonce {
    pub development: u64,
    pub production: u64,
    pub staging: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterStatus {
    pub name: String,
    pub version: String,
    pub status_at: NanoTimeStamp,
    pub canister_id: CanisterId,
    pub account_status: AccountsNonce,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: usize,
    pub canister_id: CanisterId,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Memo(pub u64);

pub type BlockIndex = u64;

#[derive(CandidType, Deserialize, Serialize)]
pub struct NotifyTopupArgs {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

#[derive(CandidType)]
pub struct AccountBalanceArgs {
    pub account: AccountIdentifier,
}

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Timestamp {
    /// Number of nanoseconds from the UNIX epoch in UTC timezone.
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct ICPTransferArgs {
    pub memo: Memo,
    pub fee: ICPToken,
    pub amount: ICPToken,
    pub to: AccountIdentifier,
    pub from_subaccount: Option<Subaccount>,
    pub created_at_time: Option<Timestamp>,
}

pub type TransferResult = Result<BlockIndex, TransferError>;

#[derive(CandidType, Deserialize, Serialize)]
pub enum NotifyTopUpResult {
    Ok(u128),
    Err(NotifyError),
}

use crate::{
    error::{HelperError, NotifyError},
    identifier::AccountIdentifier,
    subaccount::Subaccount,
    time::NanoTimeStamp,
    tokens::Tokens,
};
use ic_cdk::{
    api::management_canister::main::{CanisterInstallMode, CanisterStatusResponse},
    export::{
        candid::{CandidType, Encode},
        serde::{Deserialize, Serialize},
        Principal,
    },
};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

pub type Metadata = HashMap<String, String>;

pub type ControllerId = Principal;
pub type CanisterId = Principal;
pub type SignerId = Principal;

pub type RequestId = usize;
pub type Deadline = u64;

pub type Cycles = u128;

pub type Version = String;

pub type Blob = Vec<u8>;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Wasm(pub ByteBuf);

pub type WasmSize = usize;
pub type WasmModule = Vec<u8>;
pub type WasmHash = [u8; 32];
pub type WasmHashString = String;
pub type WasmVersion = String;

pub struct WalletCanisterInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterInitArgs {
    pub owner_id: SignerId,
    pub system_id: Option<CanisterId>,
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
pub struct TransferArgs {
    pub memo: Memo,
    pub fee: Tokens,
    pub amount: Tokens,
    pub to: AccountIdentifier,
    pub from_subaccount: Option<Subaccount>,
    pub created_at_time: Option<Timestamp>,
}

pub type TransferResult = Result<BlockIndex, HelperError>;

#[derive(CandidType, Deserialize, Serialize)]
pub enum NotifyTopUpResult {
    Ok(u128),
    Err(NotifyError),
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferFee {
    pub transfer_fee: Tokens,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferFeeArgs {}

#[derive(CandidType, Deserialize, Serialize)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize, Serialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

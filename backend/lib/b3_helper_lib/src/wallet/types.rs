use crate::{
    error::HelperError, identifier::AccountIdentifier, timestamp::NanoTimeStamp, wasm::WasmModule,
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

pub type WalletVersion = String;

pub struct WalletCanisterInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletController {
    pub name: String,
    pub metadata: Metadata,
}

impl WalletController {
    pub fn new(name: String, metadata: Option<Metadata>) -> Self {
        Self {
            name,
            metadata: metadata.unwrap_or_default(),
        }
    }
}

pub type ControllerId = Principal;
pub type ControllerIds = Vec<ControllerId>;

pub type WalletControllerMap = HashMap<ControllerId, WalletController>;

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletInititializeArgs {
    pub controllers: WalletControllerMap,
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
pub struct WalletCanisterStatus {
    pub name: String,
    pub version: String,
    pub status_at: NanoTimeStamp,
    pub canister_id: CanisterId,
    pub account_status: WalletAccountsNonce,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Default, Clone, Deserialize, Serialize)]
pub struct WalletAccountsNonce {
    pub development: u64,
    pub production: u64,
    pub staging: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: usize,
    pub canister_id: CanisterId,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType)]
pub struct ICPAccountBalanceArgs {
    pub account: AccountIdentifier,
}

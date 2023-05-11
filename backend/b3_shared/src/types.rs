use ic_cdk::{
    api::management_canister::main::CanisterStatusResponse,
    export::{candid::CandidType, serde::Deserialize, Principal},
};
use std::collections::HashMap;

pub type Metadata = HashMap<String, String>;

pub type ControllerId = Principal;
pub type CanisterId = Principal;
pub type SignerId = Principal;
pub type UserId = Principal;

pub type WasmHash = String;
pub type Version = String;

pub type Blob = Vec<u8>;
pub type Wasm = Vec<u8>;

pub struct InstallArg {
    pub wasm: Wasm,
    pub arg: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct AccountsStatus {
    pub dev_counter: u64,
    pub prod_counter: u64,
    pub stag_counter: u64,
}

#[derive(CandidType, Deserialize)]
pub struct UserControlArgs {
    pub owner: UserId,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Signer {
    pub signer_id: Option<SignerId>,
    pub created_at: u64,
    pub updated_at: u64,
    pub owner: UserId,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatus {
    pub status_at: u64,
    pub version: String,
    pub canister_id: CanisterId,
    pub accounts_status: AccountsStatus,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Subaccount(pub [u8; 32]);

#[derive(CandidType, Deserialize, Clone)]
pub struct AccountIdentifier(pub [u8; 32]);

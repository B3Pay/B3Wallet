use std::collections::HashMap;

use ic_cdk::{
    api::management_canister::main::CanisterStatusResponse,
    export::candid::{CandidType, Deserialize, Principal},
};

pub type WasmMap = HashMap<Version, Wasm>;
pub type UserControlMap = HashMap<UserId, UserControl>;
pub type Controllers = Vec<ControllerId>;
pub type Releases = Vec<Release>;
pub type Features = Vec<String>;
pub type UserControlId = Principal;
pub type ControllerId = Principal;
pub type UserId = Principal;
pub type WasmHash = String;
pub type Version = String;
pub type Blob = Vec<u8>;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserControl {
    pub user_control_id: Option<UserControlId>,
    pub created_at: u64,
    pub updated_at: u64,
    pub owner: UserId,
}

#[derive(CandidType)]
pub struct LoadRelease {
    pub total: usize,
    pub chunks: usize,
    pub version: Version,
}

#[derive(CandidType)]
pub struct UserControlArgs {
    pub owner: UserId,
}

pub struct WasmArg {
    pub wasm: Wasm,
    pub install_arg: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CanisterStatus {
    pub id: Principal,
    pub status: CanisterStatusResponse,
    pub version: String,
    pub status_at: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Wasm(pub Vec<u8>);

#[derive(CandidType, Deserialize, Clone)]
pub struct Release {
    pub date: u64,
    pub size: usize,
    pub hash: String,
    pub version: Version,
    pub deprecated: bool,
    pub features: Option<Features>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ReleaseArgs {
    pub size: usize,
    pub version: Version,
    pub features: Option<Features>,
}

use b3_helper_lib::types::{CanisterId, ControllerId, SignerId, Version, Wasm, WasmHash, WasmSize};
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

pub type SignerCanisters = Vec<SignerCanister>;
pub type Controllers = Vec<ControllerId>;
pub type Releases = Vec<Release>;
pub type Features = Vec<String>;
pub type Users = Vec<SignerId>;

pub type UserMap = HashMap<SignerId, SignerCanister>;
pub type WasmMap = HashMap<Version, Wasm>;

#[derive(CandidType, Deserialize, Clone)]
pub struct SignerCanister {
    pub canister_id: Option<CanisterId>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct State {
    pub users: UserMap,
    pub releases: Releases,
    pub controllers: Controllers,
}

#[derive(CandidType)]
pub struct LoadRelease {
    pub total: usize,
    pub chunks: usize,
    pub version: Version,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Release {
    pub date: u64,
    pub size: WasmSize,
    pub hash: WasmHash,
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

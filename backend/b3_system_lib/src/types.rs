use b3_helper::types::{ControllerId, SignerCanister, UserId, Version, Wasm, WasmHash, WasmSize};
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

pub type SignerCanisters = Vec<SignerCanister>;
pub type Controllers = Vec<ControllerId>;
pub type Releases = Vec<Release>;
pub type Features = Vec<String>;
pub type Users = Vec<UserId>;

pub type UserMap = HashMap<UserId, SignerCanister>;
pub type WasmMap = HashMap<Version, Wasm>;

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

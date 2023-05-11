use std::collections::HashMap;

use b3_shared::types::{Canister, CanisterId, UserId, Version};
use ic_cdk::export::candid::{CandidType, Deserialize};

pub type SignerMap = HashMap<UserId, Canister>;
pub type WasmMap = HashMap<Version, SystemWasm>;

pub type Controllers = Vec<UserId>;
pub type Signers = Vec<CanisterId>;
pub type Releases = Vec<Release>;
pub type Features = Vec<String>;

pub type WasmSize = usize;
pub type WasmHash = String;

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct State {
    pub signers: SignerMap,
    pub releases: Releases,
    pub controllers: Controllers,
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

#[derive(CandidType, Deserialize, Clone)]
pub struct SystemWasm(pub Vec<u8>);

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

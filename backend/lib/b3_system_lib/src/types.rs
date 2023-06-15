use b3_helper_lib::{
    release::ReleaseName,
    time::NanoTimeStamp,
    types::{CanisterId, ControllerId, SignerId, Version, Wasm, WasmHash, WasmSize},
};
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

pub type WalletCanisters = Vec<WalletCanister>;
pub type Controllers = Vec<ControllerId>;
pub type Canisters = Vec<CanisterId>;

pub type Releases = Vec<Release>;
pub type ReleaseMap = HashMap<ReleaseName, Vec<Release>>;

pub type Features = Vec<String>;
pub type Users = Vec<SignerId>;

pub type UserMap = HashMap<SignerId, WalletCanister>;
pub type WasmMap = HashMap<Version, Wasm>;

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletCanister {
    pub canisters: Vec<CanisterId>,
    pub created_at: NanoTimeStamp,
    pub updated_at: NanoTimeStamp,
}

#[derive(CandidType, Deserialize, Clone, Default)]
pub struct State {
    pub users: UserMap,
    pub releases: ReleaseMap,
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
    pub name: String,
    pub date: NanoTimeStamp,
    pub size: WasmSize,
    pub hash: WasmHash,
    pub version: Version,
    pub deprecated: bool,
    pub features: Option<Features>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ReleaseArgs {
    pub size: usize,
    pub name: String,
    pub version: Version,
    pub features: Option<Features>,
}

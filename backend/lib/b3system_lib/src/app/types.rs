use b3_utils::{
    api::AppVersion,
    ledger::Metadata,
    nonce::Nonce,
    wasm::{WasmHash, WasmHashString},
    NanoTimeStamp,
};
use candid::CandidType;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

pub type AppId = String;
pub type ReleaseId = String;

pub type ReleaseViews = Vec<ReleaseView>;

#[derive(CandidType, PartialEq, Debug)]
pub struct ReleaseView {
    pub name: String,
    pub date: NanoTimeStamp,
    pub size: usize,
    pub version: AppVersion,
    pub deprecated: bool,
    pub features: String,
    pub wasm_hash: WasmHashString,
}

#[derive(CandidType, Debug)]
pub struct AppView {
    pub app_id: AppId,
    pub name: String,
    pub description: String,
    pub created_by: String,
    pub created_at: NanoTimeStamp,
    pub updated_at: NanoTimeStamp,
    pub latest_release: Option<ReleaseView>,
    pub metadata: Metadata,
    pub install_count: Nonce,
}

#[derive(CandidType, Debug)]
pub struct LoadRelease {
    pub total: usize,
    pub chunks: usize,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateAppArgs {
    pub name: AppId,
    pub metadata: Metadata,
    pub description: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateReleaseArgs {
    pub app_id: AppId,
    pub size: usize,
    pub version: AppVersion,
    pub features: String,
    pub wasm_hash: WasmHash,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: u64,
    pub canister_status: CanisterStatusResponse,
}

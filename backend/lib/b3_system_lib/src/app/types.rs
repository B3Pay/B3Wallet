use b3_utils::{api::AppVersion, NanoTimeStamp};
use candid::CandidType;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

pub type ReleaseVersion = String;

pub type AppId = String;

pub type Features = Vec<String>;

#[derive(CandidType)]
pub struct LoadRelease {
    pub total: usize,
    pub chunks: usize,
    pub version: AppVersion,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ReleaseArgs {
    pub size: usize,
    pub name: String,
    pub version: ReleaseVersion,
    pub features: Features,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: u64,
    pub canister_status: CanisterStatusResponse,
}

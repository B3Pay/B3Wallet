use b3_utils::{
    api::AppVersion,
    types::{CanisterId, CanisterIds, UserId},
    NanoTimeStamp,
};
use candid::CandidType;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserCanisterStatus {
    pub version: String,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: u64,
    pub canister_status: CanisterStatusResponse,
}

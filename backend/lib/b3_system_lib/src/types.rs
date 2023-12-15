use b3_utils::{
    api::AppVersion,
    types::{CanisterId, CanisterIds, UserId},
    NanoTimeStamp,
};
use candid::CandidType;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

use crate::user::user::User;

pub type ReleaseVersion = String;

pub type AppId = String;

pub type UserStates = Vec<User>;

pub type Features = Vec<String>;
pub type Users = Vec<UserId>;

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
pub enum UserStatus {
    Registered,
    Unregistered,
    SingleCanister(CanisterId),
    MultipleCanister(CanisterIds),
}

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

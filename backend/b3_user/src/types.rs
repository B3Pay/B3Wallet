use b3_user_lib::types::AccountsStatus;
use ic_cdk::{
    api::management_canister::main::CanisterStatusResponse,
    export::{candid::CandidType, serde::Deserialize, Principal},
};

pub type UserId = Principal;

pub type CanisterId = Principal;

#[derive(CandidType, Deserialize)]
pub struct UserControlArgs {
    pub owner: UserId,
}

#[derive(CandidType)]
pub struct CanisterStatus {
    pub status_at: u64,
    pub version: String,
    pub canister_id: CanisterId,
    pub accounts_status: AccountsStatus,
    pub canister_status: CanisterStatusResponse,
}

use crate::{account::Account, allowance::Allowance, request::SignRequest};
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

pub type CanisterId = Principal;
pub type UserId = Principal;

pub type Metadata = HashMap<String, String>;

pub type Accounts = BTreeMap<String, Account>;

pub type CanisterAllowances = HashMap<CanisterId, Allowance>;

pub type CanisterRequests = HashMap<CanisterId, SignRequest>;

#[derive(CandidType, Deserialize)]
pub struct AccountsStatus {
    pub dev_counter: u64,
    pub prod_counter: u64,
    pub stag_counter: u64,
}

#[derive(CandidType, Deserialize)]
pub struct SetAllowance {
    pub limit: Option<u8>,
    pub metadata: Metadata,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct UserControlArgs {
    pub owner: UserId,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatus {
    pub status_at: u64,
    pub version: String,
    pub canister_id: CanisterId,
    pub accounts_status: AccountsStatus,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Default, Deserialize)]
pub enum TransactionStatus {
    #[default]
    Pending,
    Success,
    Failed,
}

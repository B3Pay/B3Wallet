use crate::{allowance::Allowance, request::SignRequest};
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::Deserialize;
use std::collections::HashMap;

pub type CanisterId = Principal;

pub type Metadata = HashMap<String, String>;

#[derive(CandidType, Deserialize, Clone)]
pub struct SetAllowance {
    pub metadata: Metadata,
    pub limit: Option<u8>,
    pub expires_at: Option<u64>,
}

pub type Allowances = HashMap<CanisterId, Allowance>;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct UserControlArgs {
    pub owner: Principal,
}

pub type CanisterHashMap = HashMap<CanisterId, Allowance>;
pub type RequestHashMap = HashMap<CanisterId, SignRequest>;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct CanisterStatus {
    pub id: Principal,
    pub version: String,
    pub status: CanisterStatusResponse,
    pub status_at: u64,
}

#[derive(Debug, CandidType, Default, Deserialize)]
pub enum TransactionStatus {
    #[default]
    Pending,
    Success,
    Failed,
}

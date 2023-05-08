use crate::{account::Account, allowance::Allowance, request::SignRequest};
use candid::{CandidType, Principal};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

pub type CanisterId = Principal;

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

#[derive(CandidType, Default, Deserialize)]
pub enum TransactionStatus {
    #[default]
    Pending,
    Success,
    Failed,
}

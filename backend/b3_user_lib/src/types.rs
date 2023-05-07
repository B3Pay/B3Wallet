use std::collections::HashMap;

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

use crate::{
    allowance::{Allowance, CanisterId},
    request::SignRequest,
};

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

#[derive(Debug, Clone, CandidType, Default, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Pending,
    Success,
    Failed,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct IcpXdrConversionRate {
    pub timestamp_seconds: u64,
    pub xdr_permyriad_per_icp: u64,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct IcpXdrConversionRateCertifiedResponse {
    pub data: IcpXdrConversionRate,
    pub hash_tree: Vec<u8>,
    pub certificate: Vec<u8>,
}

use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use crate::{transfer::TransferBlockIndex, wallet::CanisterId};

use super::NotifyError;

#[derive(CandidType, Deserialize, Serialize)]
pub struct NotifyTopupArgs {
    pub block_index: TransferBlockIndex,
    pub canister_id: CanisterId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum NotifyTopUpResult {
    Ok(u128),
    Err(NotifyError),
}

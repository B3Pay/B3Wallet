pub mod error;

use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use crate::{icp_transfer::TransferBlockIndex, wallet::CanisterId};
use error::NotifyError;

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

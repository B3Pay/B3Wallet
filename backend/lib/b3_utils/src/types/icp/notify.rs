mod error;

pub use error::*;

use crate::{types::CanisterId, types::TransferBlockIndex};
use candid::CandidType;
use serde::{Deserialize, Serialize};

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

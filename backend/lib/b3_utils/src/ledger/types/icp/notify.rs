mod error;

pub use error::*;

use crate::{ledger::types::TransferBlockIndex, types::CanisterId};
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

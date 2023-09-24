use crate::ledger::types::Balance;
use b3_utils::{ledger::ICRCAccount, Subaccount};
use candid::{CandidType, Int, Nat};
use serde::{Deserialize, Serialize};

use serde_bytes::ByteBuf;

use super::error::ICRC1TransferError;

pub type TxIndex = Nat;

pub type ICRCTokens = Nat;

pub type ICRCMetadata = Vec<(String, ICRC1MetadataValue)>;

pub type ICRCMemo = Vec<u8>;

pub type ICRCTimestamp = u64;

pub type ICRC1TransferResult = Result<TxIndex, ICRC1TransferError>;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ICRC1TransferArgs {
    pub to: ICRCAccount,
    pub amount: Balance,
    pub fee: Option<Balance>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
    pub from_subaccount: Option<Subaccount>,
}

/// Variant type for the `metadata` endpoint values.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum ICRC1MetadataValue {
    Nat(Nat),
    Int(Int),
    Text(String),
    Blob(ByteBuf),
}

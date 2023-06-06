use crate::ledger::types::Balance;
use b3_helper_lib::{
    account::ICRCAccount,
    subaccount::Subaccount,
    types::{CanisterId, Timestamp},
};
use ic_cdk::export::{
    candid::{CandidType, Int, Nat},
    serde::Deserialize,
};
use serde_bytes::ByteBuf;

pub type TxIndex = Nat;

pub type ICRCFee = Nat;

pub type ICRCMetadata = Vec<(String, ICRC1MetadataValue)>;

pub type ICRCMemo = Vec<u8>;

pub type ICRCTimestamp = u64;

#[derive(CandidType, Deserialize, Clone)]
pub struct ICRC1TransferArgs {
    pub to: ICRCAccount,
    pub amount: Balance,
    pub fee: Option<Balance>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
    pub from_subaccount: Option<Subaccount>,
}

pub enum ICRC1TransferError {
    BadFee { expected_fee: u64 },
    BadBurn { min_burn_amount: u64 },
    InsufficientFunds { balance: u64 },
    TooOld,
    CreatedInFuture { ledger_time: Timestamp },
    Duplicate { duplicate_of: u64 },
    TemporarilyUnavailable,
    GenericError { error_code: u64, message: String },
}

/// Variant type for the `metadata` endpoint values.
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ICRC1MetadataValue {
    Nat(Nat),
    Int(Int),
    Text(String),
    Blob(ByteBuf),
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ICRC {
    pub canister_id: CanisterId,
    pub subaccount: Subaccount,
    pub metadata: ICRCMetadata,
    pub fee: Option<ICRCFee>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
}

impl ICRC {
    pub fn new(
        canister_id: CanisterId,
        subaccount: Subaccount,
        fee: ICRCFee,
        metadata: ICRCMetadata,
    ) -> Self {
        ICRC {
            canister_id,
            subaccount,
            metadata,
            memo: None,
            fee: Some(fee),
            created_at_time: None,
        }
    }
}

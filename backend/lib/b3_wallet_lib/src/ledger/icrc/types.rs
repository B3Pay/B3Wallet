use crate::ledger::types::{Balance, ICRCMemo, ICRCTimestamp};
use b3_helper_lib::{
    account::ICRCAccount,
    types::{Subaccount, Timestamp},
};
use ic_cdk::export::{
    candid::{CandidType, Nat},
    serde::Deserialize,
};

pub type TxIndex = Nat;

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

pub enum ICRC1Value {
    Nat(u64),
    Int(i64),
    Text(String),
    Blob(Vec<u8>),
}

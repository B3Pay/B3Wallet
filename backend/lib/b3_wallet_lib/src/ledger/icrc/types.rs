use super::account::IcrcAccount;
use b3_helper_lib::types::{Memo, Subaccount, Timestamp};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct TransferArgs {
    to: IcrcAccount,
    amount: u64,
    fee: Option<u64>,
    memo: Option<Memo>,
    created_at_time: Option<Timestamp>,
    from_subaccount: Option<Subaccount>,
}

pub enum TransferError {
    BadFee { expected_fee: u64 },
    BadBurn { min_burn_amount: u64 },
    InsufficientFunds { balance: u64 },
    TooOld,
    CreatedInFuture { ledger_time: Timestamp },
    Duplicate { duplicate_of: u64 },
    TemporarilyUnavailable,
    GenericError { error_code: u64, message: String },
}

pub enum Value {
    Nat(u64),
    Int(i64),
    Text(String),
    Blob(Vec<u8>),
}

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::{
    allowance::CanisterId, identifier::AccountIdentifier, keys::Keys, subaccount::Subaccount,
};

pub const MAINNET_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);

pub const MAINNET_CYCLES_MINTING_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x01]);

pub type UserId = Principal;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct SignRequest {
    pub id: u64,
    pub nonce: u64,
    pub cycles: u64,
    pub data: Vec<u8>,
    pub chain_id: u64,
    pub deadline: u64,
    pub public_key: Keys,
    pub destination: CanisterId,
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
#[derive(
    CandidType, Serialize, Deserialize, Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Timestamp {
    pub timestamp_nanos: u64,
}
#[derive(
    CandidType, Serialize, Deserialize, Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Memo(pub u64);

#[derive(
    CandidType, Serialize, Deserialize, Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub memo: Memo,
    pub amount: Tokens,
    pub fee: Tokens,
    pub from_subaccount: Option<Subaccount>,
    pub to: AccountIdentifier,
    pub created_at_time: Option<Timestamp>,
}
// pub const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0; 32]);

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransferError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: BlockIndex },
}

pub type BlockIndex = u64;
pub type TransferResult = Result<BlockIndex, TransferError>;

#[derive(CandidType, Deserialize)]
pub struct NotifyTopupArgs {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum NotifyError {
    Refunded {
        block_index: Option<u64>,
        reason: String,
    },
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(u64),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum NotifyTopUpResult {
    Ok(u128),
    Err(NotifyError),
}

#[derive(Debug, CandidType, Deserialize)]
pub struct TransferFee {
    pub transfer_fee: Tokens,
}
#[derive(Debug, CandidType, Deserialize)]
pub struct TransferFeeArgs {}
#[derive(CandidType)]
pub struct AccountBalanceArgs {
    pub account: AccountIdentifier,
}

use std::collections::HashMap;

use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};

use super::{identifier::AccountIdentifier, subaccount::Subaccount};

pub type Addresses = HashMap<String, String>;

pub type Ecdsa = Vec<u8>;

pub type BlockIndex = u64;

#[derive(CandidType, Deserialize)]
pub struct NotifyTopupArgs {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

#[derive(CandidType)]
pub struct AccountBalanceArgs {
    pub account: AccountIdentifier,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tokens {
    pub e8s: u64,
}

impl Tokens {
    /// The maximum number of Tokens we can hold on a single account.
    pub const MAX: Self = Tokens { e8s: u64::MAX };
    /// Zero Tokens.
    pub const ZERO: Self = Tokens { e8s: 0 };
    /// How many times can Tokenss be divided
    pub const SUBDIVIDABLE_BY: u64 = 100_000_000;

    /// Constructs an amount of Tokens from the number of 10^-8 Tokens.
    pub const fn from_e8s(e8s: u64) -> Self {
        Self { e8s }
    }

    /// Returns the number of 10^-8 Tokens in this amount.
    pub const fn e8s(&self) -> u64 {
        self.e8s
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Timestamp {
    /// Number of nanoseconds from the UNIX epoch in UTC timezone.
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Memo(pub u64);

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub memo: Memo,
    pub fee: Tokens,
    pub amount: Tokens,
    pub to: AccountIdentifier,
    pub from_subaccount: Option<Subaccount>,
    pub created_at_time: Option<Timestamp>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum TransferError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: BlockIndex },
}

pub type TransferResult = Result<BlockIndex, TransferError>;

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

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ECDSAPublicKeyResponse {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithECDSAResponse {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct ECDSAPublicKeyArgs {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug, Deserialize)]
pub struct SignWithECDSAArgs {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

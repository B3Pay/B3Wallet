use crate::error::SharedError;
use ic_cdk::{
    api::management_canister::main::{CanisterInstallMode, CanisterStatusResponse},
    export::{
        candid::{CandidType, Encode},
        serde::Deserialize,
        Principal,
    },
};
use serde_bytes::ByteBuf;
use std::{collections::HashMap, fmt::Display};

pub type Metadata = HashMap<String, String>;

pub type ControllerId = Principal;
pub type CanisterId = Principal;
pub type UserId = Principal;

pub type WasmSize = usize;
pub type WasmModule = Vec<u8>;
pub type WasmHash = [u8; 32];

pub type Version = String;

pub type Blob = Vec<u8>;

#[derive(CandidType, Deserialize, Clone)]
pub struct Wasm(pub ByteBuf);

#[derive(CandidType, Deserialize, Clone)]
pub struct Subaccount(pub [u8; 32]);

#[derive(CandidType, Deserialize, Clone)]
pub struct AccountIdentifier(pub [u8; 32]);

#[derive(CandidType, Deserialize, Clone)]
pub struct SignerCanister {
    pub canister_id: Option<CanisterId>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct CanisterInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize)]
pub struct SignerCanisterInitArgs {
    pub owner: UserId,
}

impl SignerCanisterInitArgs {
    pub fn encode(&self) -> Result<Vec<u8>, SharedError> {
        Encode!(&self).map_err(|e| SharedError::EncodeError(e.to_string()))
    }
}

#[derive(CandidType, Deserialize)]
pub struct CanisterStatus {
    pub status_at: u64,
    pub version: String,
    pub canister_id: CanisterId,
    pub account_counter: usize,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Tokens {
    pub e8s: u64,
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e8s)
    }
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

#[derive(CandidType, Deserialize, Clone)]
pub struct Memo(pub u64);

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

#[derive(CandidType, Deserialize, Clone)]
pub struct Timestamp {
    /// Number of nanoseconds from the UNIX epoch in UTC timezone.
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransferArgs {
    pub memo: Memo,
    pub fee: Tokens,
    pub amount: Tokens,
    pub to: AccountIdentifier,
    pub from_subaccount: Option<Subaccount>,
    pub created_at_time: Option<Timestamp>,
}

pub type TransferResult = Result<BlockIndex, SharedError>;

#[derive(CandidType, Deserialize)]
pub enum NotifyTopUpResult {
    Ok(u128),
    Err(SharedError),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFee {
    pub transfer_fee: Tokens,
}

#[derive(CandidType, Deserialize)]
pub struct TransferFeeArgs {}

#[derive(CandidType, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum Environment {
    Development,
    Staging,
    #[default]
    Production,
}

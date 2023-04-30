use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::keys::Keys;

pub const MAINNET_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);

pub const MAINNET_CYCLES_MINTING_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x01]);

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct UserControlArgs {
    pub owner: Principal,
}

pub type Address = String;
pub type Satoshi = u64;
pub type MillisatoshiPerByte = u64;
pub type BlockHash = Vec<u8>;
pub type Height = u32;
pub type Page = ByteBuf;

#[derive(CandidType, Clone, Copy, Deserialize, Debug, Eq, PartialEq, Serialize, Hash)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Testnet => write!(f, "testnet"),
            Self::Regtest => write!(f, "regtest"),
        }
    }
}

/// A reference to a transaction output.
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct OutPoint {
    #[serde(with = "serde_bytes")]
    pub txid: Vec<u8>,
    pub vout: u32,
}

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct SendTransactionRequest {
    #[serde(with = "serde_bytes")]
    pub transaction: Vec<u8>,
    pub network: Network,
}

/// An unspent transaction output.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: Satoshi,
    pub height: u32,
}

/// The response returned for a request to get the UTXOs of a given address.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct GetUtxosResponse {
    pub utxos: Vec<Utxo>,
    pub tip_block_hash: BlockHash,
    pub tip_height: u32,
    pub next_page: Option<Page>,
}

/// A filter used when requesting UTXOs.
#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub enum UtxosFilter {
    MinConfirmations(u32),
    Page(Page),
}

/// A request for getting the UTXOs for a given address.
#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct GetUtxosRequest {
    pub address: Address,
    pub network: Network,
    pub filter: Option<UtxosFilter>,
}

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct GetCurrentFeePercentilesRequest {
    pub network: Network,
}

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct GetBalanceRequest {
    pub address: Address,
    pub network: Network,
    pub min_confirmations: Option<u32>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct CanisterStatus {
    pub id: Principal,
    pub status: CanisterStatusResponse,
    pub status_at: u64,
}

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

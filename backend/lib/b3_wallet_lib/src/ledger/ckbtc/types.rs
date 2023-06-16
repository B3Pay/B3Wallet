use super::error::{RetrieveBtcError, UpdateBalanceError};
use b3_helper_lib::{subaccount::Subaccount, types::CanisterId};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::fmt;

pub type BtcTxId = String;

pub type BtcTxHash = [u8; 32];

pub type Satoshi = u64;

pub type RetrieveBtcResult = Result<RetrieveBtcOk, RetrieveBtcError>;

pub type UpdateBalanceResult = Result<Vec<UtxoStatus>, UpdateBalanceError>;

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GetBtcAddressArgs {
    pub owner: Option<CanisterId>,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UpdateBalanceArgs {
    pub owner: Option<CanisterId>,
    pub subaccount: Option<Subaccount>,
}

/// A reference to a transaction output.
#[derive(
    CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct OutPoint {
    /// A cryptographic hash of the transaction.
    /// A transaction can output multiple UTXOs.
    #[serde(with = "serde_bytes")]
    pub txid: Vec<u8>,
    /// The index of the output within the transaction.
    pub vout: u32,
}

/// An unspent transaction output.
#[derive(CandidType, Debug, Deserialize, PartialEq, Serialize, Clone, Hash, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: Satoshi,
    pub height: u32,
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum UtxoStatus {
    ValueTooSmall(Utxo),
    Tainted(Utxo),
    Checked(Utxo),
    Minted {
        block_index: u64,
        minted_amount: u64,
        utxo: Utxo,
    },
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct RetrieveBtcArgs {
    // amount to retrieve in satoshi
    pub amount: u64,
    // address where to send bitcoins
    pub address: String,
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct RetrieveBtcOk {
    // the index of the burn block on the ckbtc ledger
    pub block_index: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RetrieveBtcStatusRequest {
    pub block_index: u64,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum RetrieveBtcStatus {
    Unknown,
    Pending,
    Signing,
    AmountTooLow,
    Sending { txid: BtcTxHash },
    Submitted { txid: BtcTxHash },
    Confirmed { txid: BtcTxHash },
}

impl fmt::Display for RetrieveBtcStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RetrieveBtcStatus::Unknown => write!(f, "Unknown"),
            RetrieveBtcStatus::Pending => write!(f, "Pending"),
            RetrieveBtcStatus::Signing => write!(f, "Signing"),
            RetrieveBtcStatus::AmountTooLow => write!(f, "AmountTooLow"),
            RetrieveBtcStatus::Sending { txid } => write!(f, "Sending {}", hex::encode(txid)),
            RetrieveBtcStatus::Submitted { txid } => write!(f, "Submitted {}", hex::encode(txid)),
            RetrieveBtcStatus::Confirmed { txid } => write!(f, "Confirmed {}", hex::encode(txid)),
        }
    }
}

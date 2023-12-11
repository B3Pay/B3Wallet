use crate::ledger::btc::types::{BtcTxHash, UtxoStatus};

use super::error::{RetrieveBtcError, UpdateBalanceError};
use b3_utils::{types::CanisterId, Subaccount};
use candid::{CandidType, Deserialize};

use std::fmt;

pub type RetrieveBtcResult = Result<RetrieveBtcOk, RetrieveBtcError>;

pub type UpdateBalanceResult = Result<Vec<UtxoStatus>, UpdateBalanceError>;

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct GetBtcAddressArgs {
    pub owner: Option<CanisterId>,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UpdateBalanceArgs {
    pub owner: Option<CanisterId>,
    pub subaccount: Option<Subaccount>,
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

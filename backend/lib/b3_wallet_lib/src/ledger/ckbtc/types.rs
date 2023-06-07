use b3_helper_lib::{error::ErrorTrait, subaccount::Subaccount, types::CanisterId};
use bitcoin::Txid;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

pub type BtcTxId = Txid;

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

pub enum ErrorCode {
    // The retrieval address didn't pass the KYT check.
    TaintedAddress = 1,
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
    Sending { txid: [u8; 32] },
    Submitted { txid: [u8; 32] },
    AmountTooLow,
    Confirmed { txid: [u8; 32] },
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum RetrieveBtcError {
    /// There is another request for this principal.
    AlreadyProcessing,

    /// The withdrawal amount is too low.
    AmountTooLow(u64),

    /// The bitcoin address is not valid.
    MalformedAddress(String),

    /// The withdrawal account does not hold the requested ckBTC amount.
    InsufficientFunds { balance: u64 },

    /// There are too many concurrent requests, retry later.
    TemporarilyUnavailable(String),

    /// A generic error reserved for future extensions.
    GenericError {
        error_message: String,
        /// See the [ErrorCode] enum above for the list of possible values.
        error_code: u64,
    },
}

impl ErrorTrait for RetrieveBtcError {
    fn to_string(self) -> String {
        match self {
            RetrieveBtcError::AlreadyProcessing => "AlreadyProcessing".to_string(),
            RetrieveBtcError::AmountTooLow(amount) => {
                format!("AmountTooLow({})", amount)
            }
            RetrieveBtcError::MalformedAddress(address) => {
                format!("MalformedAddress({})", address)
            }
            RetrieveBtcError::InsufficientFunds { balance } => {
                format!("InsufficientFunds({})", balance)
            }
            RetrieveBtcError::TemporarilyUnavailable(message) => {
                format!("TemporarilyUnavailable({})", message)
            }
            RetrieveBtcError::GenericError {
                error_message,
                error_code,
            } => format!("GenericError({}, {})", error_message, error_code),
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum UpdateBalanceError {
    TemporarilyUnavailable(String),
    AlreadyProcessing,
    NoNewUtxos {
        /// If there are new UTXOs that do not have enough
        /// confirmations yet, this field will contain the number of
        /// confirmations as observed by the minter.
        current_confirmations: Option<u32>,
        required_confirmations: u32,
    },
    GenericError {
        error_code: u64,
        error_message: String,
    },
}

impl ErrorTrait for UpdateBalanceError {
    fn to_string(self) -> String {
        match self {
            UpdateBalanceError::TemporarilyUnavailable(message) => {
                format!("TemporarilyUnavailable({})", message)
            }
            UpdateBalanceError::AlreadyProcessing => "AlreadyProcessing".to_string(),
            UpdateBalanceError::NoNewUtxos {
                current_confirmations,
                required_confirmations,
            } => format!(
                "NoNewUtxos({}, {})",
                current_confirmations.unwrap_or(0),
                required_confirmations
            ),
            UpdateBalanceError::GenericError {
                error_code,
                error_message,
            } => format!("GenericError({}, {})", error_code, error_message),
        }
    }
}

pub enum MinterError {
    CallError(String),
    CallResultError(),
    UpdateError(String),
    RetrieveError(RetrieveBtcError),
}

impl ErrorTrait for MinterError {
    fn to_string(self) -> String {
        match self {
            MinterError::CallError(message) => format!("CallError({})", message),
            MinterError::CallResultError() => "CallResultError".to_string(),
            MinterError::UpdateError(message) => format!("UpdateError({})", message),
            MinterError::RetrieveError(error) => format!("RetrieveError({})", error.to_string()),
        }
    }
}

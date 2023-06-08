use crate::{error::WalletError, ledger::types::Balance};
use b3_helper_lib::{
    account::ICRCAccount, error::ErrorTrait, subaccount::Subaccount, types::CanisterId,
};
use ic_cdk::export::{
    candid::{CandidType, Int, Nat},
    serde::{Deserialize, Serialize},
};
use serde_bytes::ByteBuf;

use super::icrc1::ICRC1;

pub type TxIndex = Nat;

pub type ICRCTokens = Nat;

pub type ICRCMetadata = Vec<(String, ICRC1MetadataValue)>;

pub type ICRCMemo = Vec<u8>;

pub type ICRCTimestamp = u64;

pub type ICRC1TransferResult = Result<TxIndex, ICRC1TransferError>;

#[derive(CandidType, Deserialize, Clone)]
pub struct ICRC1TransferArgs {
    pub to: ICRCAccount,
    pub amount: Balance,
    pub fee: Option<Balance>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
    pub from_subaccount: Option<Subaccount>,
}

/// Variant type for the `metadata` endpoint values.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
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
    pub fee: Option<ICRCTokens>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
}

impl ICRC {
    pub async fn new(canister_id: CanisterId, subaccount: Subaccount) -> Result<Self, WalletError> {
        let icrc1 = ICRC1(canister_id.clone());

        let metadata = icrc1
            .metadata()
            .await
            .map_err(|e| WalletError::ICRC1Error(e.to_string()))?;

        let fee = icrc1
            .fee()
            .await
            .map_err(|e| WalletError::ICRC1Error(e.to_string()))?;

        Ok(ICRC {
            canister_id,
            subaccount,
            metadata,
            memo: None,
            fee: Some(fee),
            created_at_time: None,
        })
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub enum ICRC1TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: ICRCTimestamp },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

impl ErrorTrait for ICRC1TransferError {
    fn to_string(self) -> String {
        match self {
            ICRC1TransferError::BadFee { expected_fee } => {
                format!("Bad fee: expected {}", expected_fee)
            }
            ICRC1TransferError::BadBurn { min_burn_amount } => {
                format!("Bad burn: minimum burn amount is {}", min_burn_amount)
            }
            ICRC1TransferError::InsufficientFunds { balance } => {
                format!("Insufficient funds: balance is {}", balance)
            }
            ICRC1TransferError::TooOld => "Transaction is too old".to_string(),
            ICRC1TransferError::CreatedInFuture { ledger_time } => {
                format!("Transaction created in the future: {}", ledger_time)
            }
            ICRC1TransferError::Duplicate { duplicate_of } => {
                format!("Duplicate transaction: duplicate of {}", duplicate_of)
            }
            ICRC1TransferError::TemporarilyUnavailable => "Temporarily unavailable".to_string(),
            ICRC1TransferError::GenericError {
                error_code,
                message,
            } => {
                format!("Generic error: {} - {}", error_code, message)
            }
        }
    }
}

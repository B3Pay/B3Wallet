pub mod error;

use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use crate::{account_identifier::AccountIdentifier, icp_token::ICPToken, subaccount::Subaccount};

use error::ICPTransferError;

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferMemo(pub u64);

pub type TransferBlockIndex = u64;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct ICPTransferTimestamp {
    /// Number of nanoseconds from the UNIX epoch in UTC timezone.
    pub timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct ICPTransferArgs {
    pub memo: TransferMemo,
    pub fee: ICPToken,
    pub amount: ICPToken,
    pub to: AccountIdentifier,
    pub from_subaccount: Option<Subaccount>,
    pub created_at_time: Option<ICPTransferTimestamp>,
}

pub type ICPTransferResult = Result<TransferBlockIndex, ICPTransferError>;

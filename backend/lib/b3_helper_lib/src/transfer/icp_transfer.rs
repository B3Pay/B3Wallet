use super::{ICPTransferError, TransferBlockIndex, TransferMemo};
use crate::token::ICPToken;
use crate::{identifier::AccountIdentifier, subaccount::Subaccount};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

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

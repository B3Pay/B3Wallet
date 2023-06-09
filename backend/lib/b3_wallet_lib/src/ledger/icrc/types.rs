use crate::ledger::types::Balance;
use b3_helper_lib::{account::ICRCAccount, subaccount::Subaccount, types::CanisterId};
use ic_cdk::export::{
    candid::{CandidType, Int, Nat},
    serde::{Deserialize, Serialize},
};
use serde_bytes::ByteBuf;

use super::{
    error::{ICRC1TransferError, IcrcError},
    icrc1::ICRC1,
};

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
pub struct IcrcChain {
    pub canister_id: CanisterId,
    pub subaccount: Subaccount,
    pub metadata: ICRCMetadata,
    pub fee: Option<ICRCTokens>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
}

impl IcrcChain {
    pub async fn new(canister_id: CanisterId, subaccount: Subaccount) -> Result<Self, IcrcError> {
        let icrc1 = ICRC1(canister_id.clone());

        let metadata = icrc1.metadata().await?;

        let fee = icrc1.fee().await?;

        Ok(IcrcChain {
            canister_id,
            subaccount,
            metadata,
            memo: None,
            fee: Some(fee),
            created_at_time: None,
        })
    }
}

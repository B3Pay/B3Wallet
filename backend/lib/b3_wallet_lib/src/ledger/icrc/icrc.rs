use crate::ledger::types::IcrcPending;
use b3_utils::{types::CanisterId, Subaccount};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::{
    error::IcrcError,
    icrc1::ICRC1,
    types::{ICRCMemo, ICRCMetadata, ICRCTimestamp, ICRCTokens},
};

#[derive(CandidType, Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct IcrcChain {
    pub canister_id: CanisterId,
    pub subaccount: Subaccount,
    pub metadata: ICRCMetadata,
    pub fee: Option<ICRCTokens>,
    pub memo: Option<ICRCMemo>,
    pub pendings: Vec<IcrcPending>,
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
            pendings: Vec::new(),
        })
    }
}

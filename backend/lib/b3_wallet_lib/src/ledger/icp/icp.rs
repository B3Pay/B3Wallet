use b3_helper_lib::{
    constants::{
        CANISTER_TOP_UP_MEMO, CANISTER_TRANSFER_MEMO, CYCLES_MINTING_CANISTER_ID,
        IC_TRANSACTION_FEE_ICP, LEDGER_CANISTER_ID,
    },
    identifier::AccountIdentifier,
    subaccount::Subaccount,
    tokens::Tokens,
    types::{
        CanisterId, Memo, NotifyTopUpResult, NotifyTopupArgs, Timestamp, TransferArgs,
        TransferResult,
    },
};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use crate::ledger::ledger::Ledger;

use super::error::IcpError;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct IcpChain {
    pub subaccount: Subaccount,
    pub memo: Memo,
    pub fee: Tokens,
    pub created_at_time: Option<Timestamp>,
}

impl IcpChain {
    pub fn new(subaccount: Subaccount) -> Self {
        IcpChain {
            subaccount,
            memo: CANISTER_TRANSFER_MEMO,
            fee: IC_TRANSACTION_FEE_ICP,
            created_at_time: None,
        }
    }
}

impl Ledger {
    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: Tokens,
        fee: Option<Tokens>,
        memo: Option<Memo>,
    ) -> Result<TransferResult, IcpError> {
        let args = TransferArgs {
            memo: memo.unwrap_or(CANISTER_TRANSFER_MEMO),
            fee: fee.unwrap_or(IC_TRANSACTION_FEE_ICP),
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (TransferResult,) = ic_cdk::call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| IcpError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn topup_and_notify_top_up(
        &self,
        canister_id: CanisterId,
        amount: Tokens,
        fee: Option<Tokens>,
    ) -> Result<NotifyTopUpResult, IcpError> {
        let canister_subaccount = Subaccount::from(canister_id);

        let to = AccountIdentifier::new(CYCLES_MINTING_CANISTER_ID, canister_subaccount);

        let block_index = self
            .transfer(to, amount, fee, Some(CANISTER_TOP_UP_MEMO))
            .await?
            .map_err(|e| IcpError::CallError(e.to_string()))?;

        let args = NotifyTopupArgs {
            block_index,
            canister_id,
        };

        let (res,): (NotifyTopUpResult,) =
            ic_cdk::call(CYCLES_MINTING_CANISTER_ID, "notify_top_up", (args,))
                .await
                .map_err(|e| IcpError::CallError(e.1))?;

        Ok(res)
    }
}

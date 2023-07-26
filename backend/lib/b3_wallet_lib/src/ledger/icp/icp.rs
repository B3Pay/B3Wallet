use crate::ledger::types::IcpPending;

use super::error::IcpError;
use b3_utils::{
    constants::{
        CANISTER_TOP_UP_MEMO, CANISTER_TRANSFER_MEMO, CYCLES_MINTING_CANISTER_ID,
        IC_TRANSACTION_FEE_ICP, LEDGER_CANISTER_ID,
    },
    currency::ICPToken,
    types::{
        CanisterId, ICPAccountBalanceArgs, ICPTransferArgs, ICPTransferResult,
        ICPTransferTimestamp, NotifyTopUpResult, NotifyTopupArgs, TransferBlockIndex, TransferMemo,
    },
    AccountIdentifier, Subaccount,
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct IcpChain {
    pub subaccount: Subaccount,
    pub memo: TransferMemo,
    pub fee: ICPToken,
    pub created_at_time: Option<ICPTransferTimestamp>,
    pub pendings: Vec<IcpPending>,
}

impl IcpChain {
    pub fn new(subaccount: Subaccount) -> Self {
        IcpChain {
            subaccount,
            memo: CANISTER_TRANSFER_MEMO,
            fee: IC_TRANSACTION_FEE_ICP,
            created_at_time: None,
            pendings: Vec::new(),
        }
    }
}

impl IcpChain {
    pub async fn account_balance(&self, account: AccountIdentifier) -> Result<ICPToken, IcpError> {
        let args = ICPAccountBalanceArgs { account };

        let (res,): (ICPToken,) = ic_cdk::call(LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| IcpError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: ICPToken,
        fee: Option<ICPToken>,
        memo: Option<TransferMemo>,
    ) -> Result<ICPTransferResult, IcpError> {
        let args = ICPTransferArgs {
            memo: memo.unwrap_or(self.memo.clone()),
            fee: fee.unwrap_or(self.fee.clone()),
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (ICPTransferResult,) = ic_cdk::call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| IcpError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn top_up(
        &self,
        canister_id: CanisterId,
        amount: ICPToken,
    ) -> Result<TransferBlockIndex, IcpError> {
        let canister_subaccount = Subaccount::from(canister_id);

        let to = AccountIdentifier::new(CYCLES_MINTING_CANISTER_ID, canister_subaccount);

        let block_index = self
            .transfer(to, amount, None, Some(CANISTER_TOP_UP_MEMO))
            .await?
            .map_err(IcpError::ICPTransferError)?;

        Ok(block_index)
    }

    pub async fn notify_top_up(
        &self,
        canister_id: CanisterId,
        block_index: TransferBlockIndex,
    ) -> Result<NotifyTopUpResult, IcpError> {
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

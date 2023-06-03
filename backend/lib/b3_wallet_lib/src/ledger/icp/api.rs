use async_trait::async_trait;
use b3_helper_lib::{
    constants::{
        CANISTER_TOP_UP_MEMO, CANISTER_TRANSFER_MEMO, CYCLES_MINTING_CANISTER_ID,
        IC_TRANSACTION_FEE_ICP, LEDGER_CANISTER_ID,
    },
    error::TrapError,
    types::{
        AccountBalanceArgs, AccountIdentifier, CanisterId, Memo, NotifyTopUpResult,
        NotifyTopupArgs, Subaccount, Tokens, TransferArgs, TransferResult,
    },
};
use ic_cdk::api::call::call;

use crate::{
    error::WalletError,
    ledger::types::{Balance, ChainTrait, Ledger, ICP},
};

#[async_trait]
impl ChainTrait for ICP {
    async fn balance(&self) -> Result<Balance, WalletError> {
        let account = self.identifier.clone();

        let args = AccountBalanceArgs { account };

        let (res,): (Tokens,) = call(LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res.e8s().into())
    }
}

impl Ledger {
    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: Tokens,
        fee: Option<Tokens>,
        memo: Option<Memo>,
    ) -> Result<TransferResult, WalletError> {
        let args = TransferArgs {
            memo: memo.unwrap_or(CANISTER_TRANSFER_MEMO),
            fee: fee.unwrap_or(IC_TRANSACTION_FEE_ICP),
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (TransferResult,) = call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res)
    }

    pub async fn topup_and_notify_top_up(
        &self,
        canister_id: CanisterId,
        amount: Tokens,
        fee: Option<Tokens>,
    ) -> Result<NotifyTopUpResult, WalletError> {
        let canister_subaccount = Subaccount::from(canister_id);

        let to = AccountIdentifier::new(CYCLES_MINTING_CANISTER_ID, canister_subaccount);

        let block_index = self
            .transfer(to, amount, fee, Some(CANISTER_TOP_UP_MEMO))
            .await?
            .map_err(|e| WalletError::LedgerError(e.to_string()))?;

        let args = NotifyTopupArgs {
            block_index,
            canister_id,
        };

        let (res,): (NotifyTopUpResult,) =
            call(CYCLES_MINTING_CANISTER_ID, "notify_top_up", (args,))
                .await
                .map_err(|e| WalletError::CyclesMintingError(e.1))?;

        Ok(res)
    }
}

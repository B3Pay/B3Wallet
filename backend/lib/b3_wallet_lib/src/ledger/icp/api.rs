use std::str::FromStr;

use async_trait::async_trait;
use b3_helper_lib::{
    constants::{
        CANISTER_TOP_UP_MEMO, CANISTER_TRANSFER_MEMO, CYCLES_MINTING_CANISTER_ID,
        IC_TRANSACTION_FEE_ICP, LEDGER_CANISTER_ID,
    },
    error::ErrorTrait,
    identifier::AccountIdentifier,
    subaccount::Subaccount,
    tokens::Tokens,
    types::{
        AccountBalanceArgs, CanisterId, Memo, NotifyTopUpResult, NotifyTopupArgs, TransferArgs,
        TransferResult,
    },
};
use ic_cdk::api::call::call;

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

use crate::{
    error::WalletError,
    ledger::types::{Balance, ChainTrait, Ledger, SendResult, ICP},
};

#[async_trait]
impl ChainTrait for ICP {
    fn address(&self) -> String {
        self.subaccount.account_identifier(ic_cdk_id()).to_string()
    }

    async fn balance(&self) -> Result<Balance, WalletError> {
        let canister_id = ic_cdk_id();

        let account = self.subaccount.account_identifier(canister_id);

        let args = AccountBalanceArgs { account };

        let (res,): (Tokens,) = call(LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res.e8s().into())
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, WalletError> {
        let to = AccountIdentifier::from_str(&to)
            .map_err(|e| WalletError::LedgerError(e.to_string()))?;

        let args = TransferArgs {
            memo: CANISTER_TRANSFER_MEMO,
            fee: IC_TRANSACTION_FEE_ICP,
            amount: Tokens::from_e8s(amount),
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (TransferResult,) = call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(SendResult::ICP(res))
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

use async_trait::async_trait;
use b3_helper_lib::{
    constants::{CANISTER_TRANSFER_MEMO, IC_TRANSACTION_FEE_ICP, LEDGER_CANISTER_ID},
    identifier::AccountIdentifier,
    tokens::Tokens,
    types::{AccountBalanceArgs, TransferArgs, TransferResult},
};
use std::str::FromStr;

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

use super::icp::IcpChain;
use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, SendResult},
};

#[async_trait]
impl ChainTrait for IcpChain {
    fn address(&self) -> String {
        self.subaccount.account_identifier(ic_cdk_id()).to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let canister_id = ic_cdk_id();

        let account = self.subaccount.account_identifier(canister_id);

        let args = AccountBalanceArgs { account };

        let (res,): (Tokens,) = ic_cdk::call(LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(res.e8s().into())
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError> {
        let to =
            AccountIdentifier::from_str(&to).map_err(|e| LedgerError::CallError(e.to_string()))?;

        let args = TransferArgs {
            memo: CANISTER_TRANSFER_MEMO,
            fee: IC_TRANSACTION_FEE_ICP,
            amount: Tokens::from_e8s(amount),
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (TransferResult,) = ic_cdk::call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(SendResult::ICP(res))
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        // TODO: This is a hack to get around the fact that we can't have mutable self and
        self.send(to, amount).await
    }
}

use super::types::{ICRC1TransferArgs, IcrcChain, TxIndex};
use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::account::ICRCAccount;
use std::str::FromStr;

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

#[async_trait]
impl ChainTrait for IcrcChain {
    fn address(&self) -> String {
        let owner = ic_cdk_id();

        self.subaccount.icrc_account(owner).to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let account = self.subaccount.icrc_account(ic_cdk_id());

        let (res,): (Balance,) = ic_cdk::call(self.canister_id, "icrc1_balance_of", (account,))
            .await
            .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(res)
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError> {
        let to = ICRCAccount::from_str(&to).map_err(|e| LedgerError::CallError(e.to_string()))?;

        let transfer_args = ICRC1TransferArgs {
            to,
            amount: amount.into(),
            from_subaccount: Some(self.subaccount.clone()),
            fee: self.fee.clone(),
            memo: self.memo.clone(),
            created_at_time: self.created_at_time,
        };

        let (res,): (TxIndex,) = ic_cdk::call(self.canister_id, "icrc1_transfer", (transfer_args,))
            .await
            .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(SendResult::ICRC(res))
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        // TODO: implement the update of the fee and memo fields if user wants to change them

        self.send(to, amount).await
    }
}

use super::types::{ICRC1TransferArgs, TxIndex, ICRC};
use crate::{
    error::WalletError,
    ledger::types::{Balance, ChainTrait, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{account::ICRCAccount, error::ErrorTrait};
use ic_cdk::api::call::call;
use std::str::FromStr;

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

#[async_trait]
impl ChainTrait for ICRC {
    fn address(&self) -> String {
        let owner = ic_cdk_id();

        self.subaccount.icrc1_account(owner).to_string()
    }

    async fn balance(&self) -> Result<Balance, WalletError> {
        let account = self.subaccount.icrc1_account(ic_cdk_id());

        let (res,): (Balance,) = call(self.canister_id, "icrc1_balance_of", (account,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res)
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, WalletError> {
        let to = ICRCAccount::from_str(&to).map_err(|e| WalletError::LedgerError(e.to_string()))?;

        let transfer_args = ICRC1TransferArgs {
            to,
            amount: amount.into(),
            from_subaccount: Some(self.subaccount.clone()),
            fee: self.fee.clone(),
            memo: self.memo.clone(),
            created_at_time: self.created_at_time,
        };

        let (res,): (TxIndex,) = call(self.canister_id, "icrc1_transfer", (transfer_args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(SendResult::ICRC(res))
    }
}

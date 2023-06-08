use super::ckbtc::CKBTC;
use crate::{
    error::WalletError,
    ledger::icrc::types::ICRC1TransferArgs,
    ledger::types::{Balance, ChainTrait, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{account::ICRCAccount, error::ErrorTrait};
use std::str::FromStr;

#[async_trait]
impl ChainTrait for CKBTC {
    fn address(&self) -> String {
        self.account.to_string()
    }

    async fn balance(&self) -> Result<Balance, WalletError> {
        if self.pending.is_some() {
            let result = self.update_balance().await?;

            match result {
                Ok(_) => {}
                Err(e) => return Err(WalletError::CkbtcUpdateBalance(e.to_string())),
            };
        }

        let account = self.account.clone();

        self.ledger.balance_of(account).await
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, WalletError> {
        let to = ICRCAccount::from_str(&to).map_err(|e| WalletError::LedgerError(e.to_string()))?;

        let transfer_args = ICRC1TransferArgs {
            to,
            amount: amount.into(),
            from_subaccount: self.account.subaccount(),
            fee: self.fee.clone(),
            memo: self.memo.clone(),
            created_at_time: self.created_at_time,
        };

        let result = self
            .ledger
            .transfer(transfer_args)
            .await
            .map_err(|e| WalletError::ICRC1CallError(e.to_string()))?;

        match result {
            Ok(tx_index) => Ok(SendResult::ICRC(tx_index)),
            Err(e) => Err(WalletError::LedgerError(e.to_string())),
        }
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, WalletError> {
        // TODO: update the struct if the user want that
        self.send(to, amount).await
    }
}

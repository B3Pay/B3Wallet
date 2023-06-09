use super::{ckbtc::CkbtcChain, error::CkbtcError};
use crate::{
    ledger::types::{Balance, SendResult},
    ledger::{
        chain::ChainTrait,
        error::LedgerError,
        icrc::{error::IcrcError, types::ICRC1TransferArgs},
    },
};
use async_trait::async_trait;
use b3_helper_lib::account::ICRCAccount;
use std::str::FromStr;

#[async_trait]
impl ChainTrait for CkbtcChain {
    fn address(&self) -> String {
        self.account.to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        if self.pending.is_some() {
            let result = self
                .update_balance()
                .await
                .map_err(LedgerError::CkbtcError)?;

            match result {
                Ok(_) => {}
                Err(err) => {
                    return Err(LedgerError::CkbtcError(CkbtcError::UpdateBalanceError(err)))
                }
            }
        }

        let account = self.account.clone();

        let result = self.ledger.balance_of(account).await;

        match result {
            Ok(balance) => Ok(balance),
            Err(err) => Err(LedgerError::IcrcError(err)),
        }
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError> {
        let to = ICRCAccount::from_str(&to).map_err(LedgerError::ICRCAccountError)?;

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
            .map_err(LedgerError::IcrcError)?;

        match result {
            Ok(tx_index) => Ok(SendResult::ICRC(tx_index)),
            Err(err) => Err(LedgerError::IcrcError(IcrcError::ICRC1TransferError(err))),
        }
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        // TODO: update the struct if the user want that
        self.send(to, amount).await
    }
}

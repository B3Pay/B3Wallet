use std::str::FromStr;

use crate::ledger::{
    chain::ChainTrait,
    ckbtc::minter::Minter,
    error::LedgerError,
    types::{Balance, BtcPending, PendingEnum, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{currency::TokenAmount, ICRCAccount};

use super::btc::BtcChain;

#[async_trait]
impl ChainTrait for BtcChain {
    fn address(&self) -> String {
        self.address.clone()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let address = self.address.clone();

        self.btc_network
            .get_balance(address, self.min_confirmations)
            .await
            .map_err(LedgerError::BitcoinError)
    }

    async fn send(&self, to: String, amount: TokenAmount) -> Result<SendResult, LedgerError> {
        let amount = amount
            .to_satoshi()
            .map_err(|e| LedgerError::InvalidAmountError(e.to_string()))?;

        let result = self.transfer(to, amount).await;

        match result {
            Ok(txid) => Ok(SendResult::BTC(txid)),
            Err(err) => Err(LedgerError::BitcoinError(err)),
        }
    }

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: TokenAmount,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for BTC...")
    }

    async fn check_pending(&self, pending_index: usize) -> Result<(), LedgerError> {
        let BtcPending { account, txid: _ } = self
            .pendings
            .get(pending_index)
            .ok_or(LedgerError::PendingIndexError(pending_index))?;

        let account =
            ICRCAccount::from_str(account).map_err(|e| LedgerError::ICRCAccountError(e))?;

        let result = Minter(self.btc_network).update_balance(account).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(LedgerError::CallError(err.to_string())),
        }
    }

    fn pendings(&self) -> Vec<PendingEnum> {
        self.pendings
            .iter()
            .map(|pending| PendingEnum::BTC(pending.clone()))
            .collect()
    }

    fn add_pending(&mut self, pending: PendingEnum) {
        if let PendingEnum::BTC(p) = pending {
            self.pendings.push(p);
        }
    }

    fn remove_pending(&mut self, pending_index: usize) {
        self.pendings.remove(pending_index);
    }

    fn clear_pending(&mut self) {
        self.pendings.clear();
    }
}

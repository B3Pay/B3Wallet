use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, SendResult},
};
use async_trait::async_trait;

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

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError> {
        let result = self.transfer(to, amount).await;

        match result {
            Ok(txid) => Ok(SendResult::BTC(txid)),
            Err(err) => Err(LedgerError::BitcoinError(err)),
        }
    }

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for BTC...")
    }

    fn pendings(&self) -> Vec<String> {
        self.pending.clone()
    }
}

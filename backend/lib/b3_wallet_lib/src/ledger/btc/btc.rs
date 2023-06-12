use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, SendResult},
};
use async_trait::async_trait;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use super::network::BtcNetwork;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct BtcChain {
    pub btc_network: BtcNetwork,
    pub address: String,
}

#[async_trait]
impl ChainTrait for BtcChain {
    fn address(&self) -> String {
        self.address.clone()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        self.btc_network
            .get_balance(self.address.clone(), None)
            .await
            .map_err(LedgerError::BitcoinError)
    }

    async fn send(&self, _to: String, _amount: u64) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for BTC...")
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
}

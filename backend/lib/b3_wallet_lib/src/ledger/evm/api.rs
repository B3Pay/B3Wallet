use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, ChainId, EvmPending, PendingEnum, SendResult},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use async_trait::async_trait;

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct EvmChain {
    pub chain_id: ChainId,
    pub address: String,
    pub pendings: Vec<EvmPending>,
}

#[async_trait]
impl ChainTrait for EvmChain {
    fn address(&self) -> String {
        let address = self.address.clone();

        address
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        Ok(Balance::from(0))
    }

    async fn send(&self, _to: String, _amount: u64) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn check_pending(&self, _pending_index: usize) -> Result<(), LedgerError> {
        todo!("implement the async method for EVM...")
    }

    fn pendings(&self) -> Vec<PendingEnum> {
        self.pendings
            .iter()
            .map(|pending| PendingEnum::EvmPending(pending.clone()))
            .collect()
    }

    fn add_pending(&mut self, pending: PendingEnum) {
        if let PendingEnum::EvmPending(p) = pending {
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

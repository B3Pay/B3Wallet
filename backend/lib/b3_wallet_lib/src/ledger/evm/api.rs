use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, ChainId, EvmPending, PendingEnum, SendResult},
};

use b3_utils::ledger::currency::TokenAmount;
use candid::CandidType;
use serde::{Deserialize, Serialize};

use async_trait::async_trait;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
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

    async fn send(&self, _to: String, _amount: TokenAmount) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn check_pending(&self, _pending_index: usize) -> Result<(), LedgerError> {
        todo!("implement the async method for EVM...")
    }

    fn pendings(&self) -> Vec<PendingEnum> {
        self.pendings
            .iter()
            .map(|pending| PendingEnum::EVM(pending.clone()))
            .collect()
    }

    fn add_pending(&mut self, pending: PendingEnum) {
        if let PendingEnum::EVM(p) = pending {
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

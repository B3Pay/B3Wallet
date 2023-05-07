use candid::CandidType;
use serde::Deserialize;

use crate::evm_tx::{get_evm_transaction, EvmTransaction};
use ic_cdk::api::time as ic_timestamp;

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct SignRequest {
    pub message: Vec<u8>,
    pub chain_id: u64,
    pub deadline: u64,
    pub transaction: EvmTransaction,
}

impl SignRequest {
    pub fn new(hex_raw_tx: Vec<u8>, chain_id: u64, deadline: Option<u64>) -> Self {
        let tx = get_evm_transaction(&hex_raw_tx, chain_id).unwrap();

        let message = tx.get_message_to_sign().unwrap();

        let transaction = tx.get_transaction().unwrap();

        let now = ic_timestamp();
        // now in nanoseconds since the epoch (1970-01-01)
        // and default deadline is 15 minutes
        let deadline = deadline.unwrap_or(now + 15 * 60 * 1_000_000_000);

        SignRequest {
            message,
            chain_id,
            deadline,
            transaction,
        }
    }
}

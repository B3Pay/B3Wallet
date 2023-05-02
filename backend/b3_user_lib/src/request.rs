use candid::CandidType;
use serde::Deserialize;

use crate::{
    keys::Keys,
    transaction::{get_transaction, Transaction},
};

#[derive(Debug, CandidType, Deserialize)]
pub struct SignRequest {
    pub message: Vec<u8>,
    pub chain_id: u64,
    pub deadline: u64,
    pub public_key: Keys,
    pub transaction: Transaction,
}

impl SignRequest {
    pub fn new(hex_raw_tx: Vec<u8>, chain_id: u64) -> Self {
        let tx = get_transaction(&hex_raw_tx, chain_id).unwrap();

        let message = tx.get_message_to_sign().unwrap();

        let transaction = tx.get_transaction().unwrap();

        todo!();
    }
}

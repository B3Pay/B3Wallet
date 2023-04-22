use crate::{derivation::Derivation, transaction::get_transaction};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use std::collections::HashMap;

use crate::{
    chain::ChainData, config::Environment, public_key::PublicKey, signed::SignedTransaction,
};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    name: String,
    public_key: PublicKey,
    derivation: Derivation,
    chain_data: HashMap<u64, ChainData>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            name: String::new(),
            public_key: PublicKey::default(),
            derivation: Derivation::default(),
            chain_data: HashMap::new(),
        }
    }
}

impl Account {
    pub async fn new(path: Vec<u8>, env: Environment) -> Self {
        let derivation = Derivation::new(path, env);

        let bytes = derivation.public_key().await;

        Account {
            derivation,
            name: String::new(),
            chain_data: HashMap::new(),
            public_key: PublicKey::new(bytes),
        }
    }

    pub async fn new_transaction(&self, hex_raw_tx: Vec<u8>, chain_id: u64) -> SignedTransaction {
        let mut tx = get_transaction(&hex_raw_tx, chain_id).unwrap();

        let message = tx.get_message_to_sign().unwrap();

        assert!(message.len() == 32);

        let signature = self.derivation.sign_message(message).await;

        let signed_tx = tx.sign(signature, self.public_key.key()).unwrap();

        SignedTransaction::new(signed_tx)
    }

    pub fn insert_transaction(&mut self, chain_id: u64, transaction: SignedTransaction) {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data
                .get_mut(&chain_id)
                .unwrap()
                .add_transaction(transaction);
        } else {
            let mut chain_data = ChainData::default();
            chain_data.add_transaction(transaction);

            self.chain_data.insert(chain_id, chain_data);
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn remove_transactions(&mut self, chain_id: u64) {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data.remove(&chain_id);
        }
    }

    pub fn remove_transaction(&mut self, chain_id: u64, index: u64) {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data
                .get_mut(&chain_id)
                .unwrap()
                .remove_transaction(index);
        }
    }

    pub fn nonce(&self, chain_id: u64) -> u64 {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data.get(&chain_id).unwrap().nonce()
        } else {
            0
        }
    }

    pub fn public_data(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn transactions(&self, chain_id: u64) -> Vec<SignedTransaction> {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data.get(&chain_id).unwrap().transactions()
        } else {
            vec![]
        }
    }

    pub fn transaction(&self, chain_id: u64, index: u64) -> Option<SignedTransaction> {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data.get(&chain_id).unwrap().transaction(index)
        } else {
            None
        }
    }

    pub fn last_transaction(&self, chain_id: u64) -> Option<SignedTransaction> {
        if self.chain_data.contains_key(&chain_id) {
            self.chain_data.get(&chain_id).unwrap().last_transaction()
        } else {
            None
        }
    }
}

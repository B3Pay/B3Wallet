use crate::{ecdsa::Ecdsa, transaction::get_transaction};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use std::collections::HashMap;

use crate::{chain::Chain, config::Environment, keys::Keys, signed::SignedTransaction};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    name: String,
    keys: Keys,
    ecdsa: Ecdsa,
    chains: HashMap<u64, Chain>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            name: String::new(),
            keys: Keys::default(),
            ecdsa: Ecdsa::default(),
            chains: HashMap::new(),
        }
    }
}

impl Account {
    pub async fn new(path: Vec<u8>, env: Environment) -> Self {
        let ecdsa = Ecdsa::new(path, env);

        let bytes = ecdsa.public_key().await;

        Account {
            ecdsa,
            name: String::new(),
            keys: Keys::new(bytes),
            chains: HashMap::new(),
        }
    }

    pub async fn new_transaction(&self, hex_raw_tx: Vec<u8>, chain_id: u64) -> SignedTransaction {
        let mut tx = get_transaction(&hex_raw_tx, chain_id).unwrap();

        let message = tx.get_message_to_sign().unwrap();

        assert!(message.len() == 32);

        let signature = self.ecdsa.sign_message(message).await;

        let signed_tx = tx.sign(signature, self.keys.key()).unwrap();

        SignedTransaction::new(signed_tx)
    }

    pub fn insert_transaction(&mut self, chain_id: u64, transaction: SignedTransaction) {
        if self.chains.contains_key(&chain_id) {
            self.chains
                .get_mut(&chain_id)
                .unwrap()
                .add_transaction(transaction);
        } else {
            let mut chain_data = Chain::default();
            chain_data.add_transaction(transaction);

            self.chains.insert(chain_id, chain_data);
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn remove_chain(&mut self, chain_id: u64) {
        if self.chains.contains_key(&chain_id) {
            self.chains.remove(&chain_id);
        }
    }

    pub fn remove_transaction(&mut self, chain_id: u64, index: u64) {
        if self.chains.contains_key(&chain_id) {
            self.chains
                .get_mut(&chain_id)
                .unwrap()
                .remove_transaction(index);
        }
    }

    pub fn nonce(&self, chain_id: u64) -> u64 {
        if self.chains.contains_key(&chain_id) {
            self.chains.get(&chain_id).unwrap().nonce()
        } else {
            0
        }
    }

    pub fn keys(&self) -> Keys {
        self.keys.clone()
    }

    pub fn transactions(&self, chain_id: u64) -> Vec<SignedTransaction> {
        if self.chains.contains_key(&chain_id) {
            self.chains.get(&chain_id).unwrap().transactions()
        } else {
            vec![]
        }
    }

    pub fn transaction(&self, chain_id: u64, index: u64) -> Option<SignedTransaction> {
        if self.chains.contains_key(&chain_id) {
            self.chains.get(&chain_id).unwrap().transaction(index)
        } else {
            None
        }
    }

    pub fn last_transaction(&self, chain_id: u64) -> Option<SignedTransaction> {
        if self.chains.contains_key(&chain_id) {
            self.chains.get(&chain_id).unwrap().last_transaction()
        } else {
            None
        }
    }
}

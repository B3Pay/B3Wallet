use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::signed::SignedTransaction;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Chain {
    nonce: u64,
    transactions: Vec<SignedTransaction>,
}

impl Default for Chain {
    fn default() -> Self {
        Chain {
            nonce: 0 as u64,
            transactions: vec![],
        }
    }
}

impl Chain {
    pub fn add_transaction(&mut self, transaction: SignedTransaction) {
        self.increment_nonce();
        self.transactions.push(transaction);
    }

    pub fn update_transaction(&mut self, index: u64, transaction: SignedTransaction) {
        if index < self.transactions.len() as u64 {
            self.transactions[index as usize] = transaction;
        }
    }

    pub fn remove_transaction(&mut self, index: u64) {
        if index < self.transactions.len() as u64 {
            self.transactions.remove(index as usize);
        }
    }

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn transactions(&self) -> Vec<SignedTransaction> {
        self.transactions.clone()
    }

    pub fn transaction(&self, index: u64) -> Option<SignedTransaction> {
        if index < self.transactions.len() as u64 {
            Some(self.transactions[index as usize].clone())
        } else {
            None
        }
    }

    pub fn last_transaction(&self) -> Option<SignedTransaction> {
        if self.transactions.len() > 0 {
            Some(self.transactions[self.transactions.len() - 1].clone())
        } else {
            None
        }
    }
}

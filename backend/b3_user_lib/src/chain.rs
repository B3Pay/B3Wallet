use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::signed::SignedTransaction;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ChainData {
    nonce: u64,
    transactions: Vec<SignedTransaction>,
}

impl Default for ChainData {
    fn default() -> Self {
        ChainData {
            nonce: 0 as u64,
            transactions: vec![],
        }
    }
}

impl ChainData {
    pub fn add_transaction(&mut self, transaction: SignedTransaction) {
        self.nonce += 1;
        self.transactions.push(transaction);
    }

    pub fn transactions(&self) -> Vec<SignedTransaction> {
        self.transactions.clone()
    }

    pub fn remove_transaction(&mut self, index: u64) {
        if index < self.transactions.len() as u64 {
            self.transactions.remove(index as usize);
        }
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
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

    pub fn last_transaction_timestamp(&self) -> u64 {
        if self.transactions.len() > 0 {
            self.transactions[self.transactions.len() - 1].timestamp
        } else {
            0
        }
    }

    pub fn get_last_transaction_data(&self) -> Vec<u8> {
        if self.transactions.len() > 0 {
            self.transactions[self.transactions.len() - 1].data.clone()
        } else {
            vec![]
        }
    }

    pub fn get_last_transaction_data_as_string(&self) -> String {
        if self.transactions.len() > 0 {
            String::from_utf8(self.transactions[self.transactions.len() - 1].data.clone()).unwrap()
        } else {
            String::from("")
        }
    }
}

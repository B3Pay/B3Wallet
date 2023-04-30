use ic_cdk::{
    export::{candid::CandidType, serde::Deserialize},
    trap,
};

use crate::{
    btc::{get_balance, send_transaction},
    types::Network,
    utils::{get_address_from_public_key, get_p2pkh_address_from_public_key},
};

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Keys {
    bytes: Vec<u8>,
    eth_address: String,
    btc_address: String,
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            bytes: vec![],
            eth_address: String::new(),
            btc_address: String::new(),
        }
    }
}

impl Keys {
    pub fn new(bytes: Vec<u8>) -> Self {
        let eth_address = get_address_from_public_key(bytes.clone())
            .map_err(|e| trap(&format!("Failed to get address from public key {}", e)))
            .unwrap();

        let btc_address = get_p2pkh_address_from_public_key(Network::Regtest, bytes.clone())
            .map_err(|e| {
                trap(&format!(
                    "Failed to get p2pkh address from public key {}",
                    e
                ))
            })
            .unwrap();

        Keys {
            bytes,
            eth_address,
            btc_address,
        }
    }

    pub async fn btc_balance(&self) -> u64 {
        get_balance(Network::Regtest, self.btc_address.clone()).await
    }

    pub async fn btc_send(&self, transaction: Vec<u8>) -> Result<(), String> {
        send_transaction(Network::Regtest, transaction).await;

        Ok(())
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn address(&self) -> String {
        self.eth_address.clone()
    }
}

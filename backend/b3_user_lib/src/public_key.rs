use ic_cdk::{
    export::{candid::CandidType, serde::Deserialize},
    trap,
};

use crate::utils::get_address_from_public_key;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct PublicKey {
    bytes: Vec<u8>,
    address: String,
}

impl Default for PublicKey {
    fn default() -> Self {
        PublicKey {
            bytes: vec![],
            address: "".to_string(),
        }
    }
}

impl PublicKey {
    pub fn new(bytes: Vec<u8>) -> Self {
        let address = get_address_from_public_key(bytes.clone())
            .map_err(|e| trap(&format!("Failed to get address from public key {}", e)))
            .unwrap();

        PublicKey { bytes, address }
    }

    pub fn key(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}

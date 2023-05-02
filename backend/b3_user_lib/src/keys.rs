use ic_cdk::{
    export::{candid::CandidType, serde::Deserialize},
    trap,
};

use crate::{
    types::Network,
    utils::{get_address_from_public_key, get_p2pkh_address_from_public_key},
};

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Addresses {
    eth: String,
    btc: BtcAddresses,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct BtcAddresses {
    mainnet: String,
    testnet: String,
}

impl Default for Addresses {
    fn default() -> Self {
        Addresses {
            eth: String::new(),
            btc: BtcAddresses {
                mainnet: String::new(),
                testnet: String::new(),
            },
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Keys {
    bytes: Vec<u8>,
    addresses: Addresses,
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            bytes: vec![],
            addresses: Addresses::default(),
        }
    }
}

impl Keys {
    pub fn new(bytes: Vec<u8>) -> Self {
        let eth = get_address_from_public_key(bytes.clone())
            .map_err(|e| trap(&format!("Failed to get address from public key {}", e)))
            .unwrap();

        let mainnet = get_p2pkh_address_from_public_key(Network::Mainnet, bytes.clone()).unwrap();

        let testnet = get_p2pkh_address_from_public_key(Network::Regtest, bytes.clone()).unwrap();

        Keys {
            bytes,
            addresses: Addresses {
                eth,
                btc: BtcAddresses { mainnet, testnet },
            },
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn addresses(&self) -> Addresses {
        self.addresses.clone()
    }
}

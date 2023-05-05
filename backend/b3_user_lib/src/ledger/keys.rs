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
    icp: String,
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
            icp: String::new(),
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
    pub bytes: Vec<u8>,
    pub addresses: Addresses,
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
    pub fn new(bytes: Vec<u8>, icp: String) -> Self {
        let eth = get_address_from_public_key(&bytes)
            .map_err(|e| trap(&format!("Failed to get address from public key {}", e)))
            .unwrap();

        let mainnet = get_p2pkh_address_from_public_key(Network::Mainnet, &bytes).unwrap();

        let testnet = get_p2pkh_address_from_public_key(Network::Regtest, &bytes).unwrap();

        Keys {
            bytes,
            addresses: Addresses {
                icp,
                eth,
                btc: BtcAddresses { mainnet, testnet },
            },
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

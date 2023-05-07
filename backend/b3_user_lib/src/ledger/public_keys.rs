use easy_hasher::easy_hasher;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::{
    error::SignerError,
    types::{BitcoinNetwork, Network},
};

use super::{
    identifier::AccountIdentifier,
    subaccount::Subaccount,
    types::{Addresses, Ecdsa},
};

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct PublicKeys {
    pub ecdsa: Option<Ecdsa>,
    pub identifier: AccountIdentifier,
    pub addresses: Addresses,
}

impl Default for PublicKeys {
    fn default() -> Self {
        PublicKeys {
            ecdsa: None,
            addresses: HashMap::new(),
            identifier: AccountIdentifier::default(),
        }
    }
}

impl PublicKeys {
    pub fn new(subaccount: &Subaccount) -> Self {
        let identifier = subaccount.get_account_identifier();

        PublicKeys {
            ecdsa: None,
            identifier,
            addresses: HashMap::new(),
        }
    }

    pub fn set_ecdsa(&mut self, ecdsa: Vec<u8>) -> Result<Vec<u8>, SignerError> {
        if ecdsa.len() != 33 {
            return Err(SignerError::InvalidEcdsaPublicKey);
        }

        self.ecdsa = Some(ecdsa.clone());

        Ok(ecdsa)
    }

    pub fn get_ecdsa(&self) -> Result<Vec<u8>, SignerError> {
        match &self.ecdsa {
            Some(ecdsa) => Ok(ecdsa.clone()),
            None => Err(SignerError::MissingEcdsaPublicKey),
        }
    }

    pub fn get_identifier(&self) -> AccountIdentifier {
        self.identifier.clone()
    }

    pub fn get_addresses(&self) -> HashMap<String, String> {
        self.addresses.clone()
    }

    pub fn generate_eth_address(&mut self) -> Result<String, SignerError> {
        let ecdsa = self.get_ecdsa()?;

        let pub_key_arr: [u8; 33] = ecdsa[..].try_into().unwrap();
        let pub_key = libsecp256k1::PublicKey::parse_compressed(&pub_key_arr)
            .map_err(|e| SignerError::GenerateError(e.to_string()))?
            .serialize();

        let keccak256 = easy_hasher::raw_keccak256(pub_key[1..].to_vec());
        let keccak256_hex = keccak256.to_hex_string();
        let address: String = "0x".to_owned() + &keccak256_hex[24..];

        self.addresses
            .insert(Network::Ethereum.to_string(), address.clone());

        Ok(address)
    }

    pub fn generate_btc_address(
        &mut self,
        bitcoin_network: BitcoinNetwork,
    ) -> Result<String, SignerError> {
        let bytes = self.get_ecdsa()?;

        let mut hasher = Ripemd160::new();
        hasher.update(bytes);
        let result = hasher.finalize();

        let prefix = match bitcoin_network {
            BitcoinNetwork::Mainnet => 0x00,
            _ => 0x6f,
        };

        let mut data_with_prefix = vec![prefix];
        data_with_prefix.extend(result);

        let checksum = &sha256(&sha256(&data_with_prefix.clone()))[..4];

        let mut full_address = data_with_prefix;
        full_address.extend(checksum);

        let address: String = bs58::encode(full_address).into_string();

        self.addresses
            .insert(bitcoin_network.to_string(), address.clone());

        Ok(address)
    }
}

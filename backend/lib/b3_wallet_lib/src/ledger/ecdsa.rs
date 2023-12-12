use bitcoin::{secp256k1, Address, Network, PublicKey};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::error::LedgerError;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct ECDSAPublicKey(pub [u8; 32]);

impl ECDSAPublicKey {
    pub fn new(ecdsa: Vec<u8>) -> Self {
        let mut ecdsa_array = [0u8; 32];

        ecdsa_array.copy_from_slice(&ecdsa);

        Self(ecdsa_array)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, LedgerError> {
        if slice.len() != 33 {
            return Err(LedgerError::PublicKeyError(
                "Invalid public key length".to_string(),
            ));
        }

        Ok(Self::new(slice.to_vec()))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0x02];

        bytes.extend_from_slice(&self.0);

        bytes
    }

    pub fn btc_public_key(&self) -> Result<PublicKey, LedgerError> {
        PublicKey::from_slice(&self.0).map_err(|err| LedgerError::PublicKeyError(err.to_string()))
    }

    /// Get the Bitcoin P2PKH Address based on the public key.
    /// This is the address that the canister uses to send and receive funds.
    pub fn btc_address(&self, network: Network) -> Result<Address, LedgerError> {
        let public_key = self.btc_public_key()?;

        let address = Address::p2pkh(&public_key, network);

        Ok(address)
    }

    pub fn to_secp256k1_public_key(&self) -> Result<secp256k1::PublicKey, LedgerError> {
        let public_key = self.btc_public_key()?;

        Ok(public_key.inner)
    }
}

use bitcoin::{secp256k1, Address, Network, PublicKey};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use super::error::LedgerError;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct EcdsaPublicKey(pub Vec<u8>);

impl EcdsaPublicKey {
    pub fn new(ecdsa: Vec<u8>) -> Self {
        Self(ecdsa)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, LedgerError> {
        if slice.len() != 33 {
            return Err(LedgerError::PublicKeyError(
                "Invalid public key length".to_string(),
            ));
        }

        Ok(Self(slice.to_vec()))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn btc_public_key(&self) -> Result<PublicKey, LedgerError> {
        PublicKey::from_slice(&self.0).map_err(|err| LedgerError::PublicKeyError(err.to_string()))
    }

    /// Get the Bitcoin P2WPKH Address based on the public key.
    /// This is the address that the canister uses to send and receive funds.
    pub fn btc_address(&self, network: Network) -> Result<Address, LedgerError> {
        let public_key = self.btc_public_key()?;

        let address = Address::p2wpkh(&public_key, network)
            .map_err(|err| LedgerError::PublicKeyError(err.to_string()))?;

        Ok(address)
    }

    pub fn to_secp256k1_public_key(&self) -> Result<secp256k1::PublicKey, LedgerError> {
        let public_key = self.btc_public_key()?;

        Ok(public_key.inner)
    }
}

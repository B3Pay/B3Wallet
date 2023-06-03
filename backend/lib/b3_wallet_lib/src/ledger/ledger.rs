use super::{
    btc::network::BtcNetwork,
    subaccount::SubaccountTrait,
    types::{Balance, Chain, ChainId, ChainMap, ChainType, Ledger},
};
use crate::{error::WalletError, ledger::types::ChainTrait};
use b3_helper_lib::{
    raw_keccak256,
    types::{CanisterId, Environment, Subaccount},
};
use bitcoin::{secp256k1, Address, PublicKey};
use std::collections::BTreeMap;

impl Default for Ledger {
    fn default() -> Self {
        Ledger {
            ecdsa: None,
            chains: BTreeMap::default(),
            subaccount: Subaccount::default(),
        }
    }
}

impl From<Subaccount> for Ledger {
    fn from(subaccount: Subaccount) -> Self {
        let canister_id = ic_cdk::id();

        let identifier = subaccount.account_identifier(canister_id);

        let ic_chain = Chain::new_icp_chain(identifier);

        let mut chains = ChainMap::new();

        chains.insert(ChainType::ICP, ic_chain);

        Ledger {
            ecdsa: None,
            subaccount,
            chains,
        }
    }
}

impl Ledger {
    pub fn is_ecdsa_set(&self) -> bool {
        self.ecdsa
            .as_ref()
            .map(|ecdsa| ecdsa.len() == 33)
            .unwrap_or(false)
    }

    pub fn addresses(&self) -> &ChainMap {
        &self.chains
    }

    pub fn ecdsa(&self) -> Result<&Vec<u8>, WalletError> {
        match &self.ecdsa {
            Some(ecdsa) => Ok(ecdsa),
            None => Err(WalletError::MissingEcdsaPublicKey),
        }
    }

    pub fn public_key(&self) -> Result<PublicKey, WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|_| WalletError::InvalidEcdsaPublicKey)?;

        Ok(public_key)
    }

    pub fn chain(&self, chains: ChainType) -> Result<Chain, WalletError> {
        self.chains
            .get(&chains)
            .cloned()
            .ok_or_else(|| WalletError::MissingAddress)
    }

    pub fn icp_chain(&self) -> Result<Chain, WalletError> {
        self.chain(ChainType::ICP)
    }

    pub fn icrc_chain(&self, canister_id: CanisterId) -> Result<Chain, WalletError> {
        self.chain(ChainType::ICRC(canister_id))
    }

    pub fn eth_address(&self) -> Result<String, WalletError> {
        self.derive_eth_address()
    }

    pub fn btc_address(&self, btc_network: BtcNetwork) -> Result<Address, WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|e| WalletError::GenerateError(e.to_string()))?;

        let address = Address::p2pkh(&public_key, btc_network.into());

        Ok(address)
    }

    pub async fn get_balance(&self, chain_type: ChainType) -> Result<Balance, WalletError> {
        match self.chains.get(&chain_type) {
            Some(chain) => chain.balance().await,
            None => Err(WalletError::MissingAddress),
        }
    }

    pub fn set_ecdsa(&mut self, ecdsa: Vec<u8>) -> Result<(), WalletError> {
        if ecdsa.len() != 33 {
            return Err(WalletError::InvalidEcdsaPublicKey);
        }

        if self.is_ecdsa_set() {
            return Err(WalletError::EcdsaPublicKeyAlreadySet);
        }

        let env = self.subaccount.environment();

        let ecdsa = PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .to_bytes();

        self.ecdsa = Some(ecdsa);

        match env {
            Environment::Production => {
                self.generate_address(ChainType::EVM(1))?;
                self.generate_address(ChainType::BTC(BtcNetwork::Mainnet))?;
            }
            Environment::Staging => {
                self.generate_address(ChainType::EVM(137))?;
                self.generate_address(ChainType::BTC(BtcNetwork::Testnet))?;
            }
            Environment::Development => {
                self.generate_address(ChainType::EVM(5))?;
                self.generate_address(ChainType::BTC(BtcNetwork::Regtest))?;
            }
        }

        Ok(())
    }

    pub fn generate_address(&mut self, chains: ChainType) -> Result<(), WalletError> {
        match chains {
            ChainType::BTC(btc_network) => self.generate_btc_address(btc_network),
            ChainType::ICRC(token) => self.generate_icrc_address(token),
            ChainType::EVM(chain) => self.generate_eth_address(chain),
            ChainType::ICP => Ok(()),
        }
    }

    pub fn generate_icp_address(&mut self) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        let identifier = self.subaccount.account_identifier(canister_id);

        let icp_chain = Chain::new_icp_chain(identifier);

        self.chains.insert(ChainType::ICP, icp_chain);

        Ok(())
    }

    pub fn generate_icrc_address(&mut self, canister_id: CanisterId) -> Result<(), WalletError> {
        let icrc_chain = Chain::new_icrc_chain(canister_id, self.subaccount.clone());

        self.chains.insert(ChainType::ICRC(canister_id), icrc_chain);

        Ok(())
    }

    pub fn generate_eth_address(&mut self, chain_id: ChainId) -> Result<(), WalletError> {
        let address = self.derive_eth_address()?;

        let eth_chain = Chain::new_evm_chain(chain_id, address);

        self.chains.insert(ChainType::EVM(chain_id), eth_chain);

        Ok(())
    }

    pub fn generate_btc_address(&mut self, btc_network: BtcNetwork) -> Result<(), WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|e| WalletError::GenerateError(e.to_string()))?;

        let address = Address::p2pkh(&public_key, btc_network.into()).to_string();

        let btc_chain = Chain::new_btc_chain(btc_network, address);

        self.chains.insert(ChainType::BTC(btc_network), btc_chain);

        Ok(())
    }

    pub fn remove_address(&mut self, chains: ChainType) -> Result<(), WalletError> {
        if self.chains.remove(&chains).is_none() {
            return Err(WalletError::MissingAddress);
        }

        Ok(())
    }

    fn derive_eth_address(&self) -> Result<String, WalletError> {
        let ecdsa = self.ecdsa()?;

        let pub_key = secp256k1::PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .serialize_uncompressed();

        let keccak256 = raw_keccak256(&pub_key[1..]);

        let keccak256_hex = keccak256.to_hex_string();

        let address: String = "0x".to_owned() + &keccak256_hex[24..];

        Ok(address)
    }
}

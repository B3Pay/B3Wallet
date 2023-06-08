use super::{
    btc::network::BtcNetwork,
    chain::Chain,
    ckbtc::ckbtc::CKBTC,
    icrc::types::ICRC,
    types::{AddressMap, Balance, ChainEnum, ChainId, ChainMap, EcdsaPublicKey, SendResult},
};
use crate::{error::WalletError, ledger::types::ChainTrait};
use b3_helper_lib::{raw_keccak256, subaccount::Subaccount, types::CanisterId};
use bitcoin::{secp256k1, Address, PublicKey};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Ledger {
    pub ecdsa: Option<EcdsaPublicKey>,
    pub subaccount: Subaccount,
    pub chains: ChainMap,
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger {
            ecdsa: None,
            chains: ChainMap::default(),
            subaccount: Subaccount::default(),
        }
    }
}

impl From<Subaccount> for Ledger {
    fn from(subaccount: Subaccount) -> Self {
        let ic_chain = Chain::new_icp_chain(subaccount.clone());

        let mut chains = ChainMap::new();

        chains.insert(ChainEnum::ICP, ic_chain);

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

    pub async fn send(
        &self,
        chain_type: ChainEnum,
        to: String,
        amount: u64,
    ) -> Result<SendResult, WalletError> {
        let chain = self.chain(chain_type)?;

        chain.send(to, amount).await
    }
    pub async fn send_mut(
        &mut self,
        chain_type: ChainEnum,
        to: String,
        amount: u64,
        fee: Option<u64>,
        memo: Option<String>,
    ) -> Result<SendResult, WalletError> {
        let chain = self.chain_mut(chain_type)?;

        chain.send_mut(to, amount, fee, memo).await
    }

    pub async fn balance(&self, chain_type: ChainEnum) -> Result<Balance, WalletError> {
        match self.chains.get(&chain_type) {
            Some(chain) => chain.balance().await,
            None => Err(WalletError::MissingAddress),
        }
    }

    pub fn addresses(&self) -> AddressMap {
        let mut addresses = AddressMap::new();

        for (chain_type, chain) in &self.chains {
            let address = chain.address();

            addresses.insert(chain_type.clone(), address);
        }

        addresses
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

    pub fn chain(&self, chains: ChainEnum) -> Result<&Chain, WalletError> {
        self.chains
            .get(&chains)
            .ok_or_else(|| WalletError::MissingAddress)
    }

    pub fn chain_mut(&mut self, chains: ChainEnum) -> Result<&mut Chain, WalletError> {
        self.chains
            .get_mut(&chains)
            .ok_or_else(|| WalletError::MissingAddress)
    }

    pub fn eth_address(&self) -> Result<String, WalletError> {
        let ecdsa = self.ecdsa()?;

        let pub_key = secp256k1::PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .serialize_uncompressed();

        let keccak256 = raw_keccak256(&pub_key[1..]);

        let keccak256_hex = keccak256.to_hex_string();

        let address = "0x".to_owned() + &keccak256_hex[24..];

        Ok(address)
    }

    pub fn btc_address(&self, btc_network: BtcNetwork) -> Result<Address, WalletError> {
        let public_key = self.public_key()?;

        let address = Address::p2pkh(&public_key, btc_network.into());

        Ok(address)
    }

    pub async fn new_chain(&self, chain_type: ChainEnum) -> Result<Chain, WalletError> {
        match chain_type {
            ChainEnum::CKBTC(btc_network) => {
                let subaccount = self.subaccount.to_owned();
                let chain = Chain::new_ckbtc_chain(btc_network, subaccount).await?;

                Ok(chain)
            }
            ChainEnum::ICRC(canister_id) => {
                let subaccount = self.subaccount.to_owned();
                let chain = Chain::new_icrc_chain(canister_id, subaccount).await?;

                Ok(chain)
            }
            ChainEnum::BTC(btc_network) => {
                let btc_address = self.btc_address(btc_network)?;

                let btc_chain = Chain::new_btc_chain(btc_network, btc_address.to_string());

                Ok(btc_chain)
            }
            ChainEnum::EVM(chain_id) => {
                let eth_address = self.eth_address()?;

                let eth_chain = Chain::new_evm_chain(chain_id, eth_address);

                Ok(eth_chain)
            }
            ChainEnum::ICP => {
                let icp_chain = Chain::new_icp_chain(self.subaccount.to_owned());

                Ok(icp_chain)
            }
        }
    }

    pub fn icrc(&self, canister_id: CanisterId) -> Option<&ICRC> {
        let chain = self.chains.get(&ChainEnum::ICRC(canister_id))?;

        chain.icrc()
    }

    pub fn icrc_mut(&mut self, canister_id: CanisterId) -> Option<&mut ICRC> {
        let chain = self.chains.get_mut(&ChainEnum::ICRC(canister_id))?;

        chain.icrc_mut()
    }

    pub fn ckbtc(&self, network: BtcNetwork) -> Option<&CKBTC> {
        let chain = self.chains.get(&ChainEnum::CKBTC(network))?;

        chain.ckbtc()
    }

    pub fn ckbtc_mut(&mut self, network: BtcNetwork) -> Option<&mut CKBTC> {
        let chain = self.chains.get_mut(&ChainEnum::CKBTC(network))?;

        chain.ckbtc_mut()
    }

    pub fn icp_chain(&self) -> Chain {
        Chain::new_icp_chain(self.subaccount.to_owned())
    }

    pub fn eth_chain(&self, chain_id: ChainId) -> Result<Chain, WalletError> {
        let eth_address = self.eth_address()?;

        let eth_chain = Chain::new_evm_chain(chain_id, eth_address);

        Ok(eth_chain)
    }

    pub fn btc_chain(&self, btc_network: BtcNetwork) -> Result<Chain, WalletError> {
        let btc_address = self.btc_address(btc_network)?;

        let btc_chain = Chain::new_btc_chain(btc_network, btc_address.to_string());

        Ok(btc_chain)
    }

    pub fn set_ecdsa(&mut self, ecdsa: Vec<u8>) -> Result<(), WalletError> {
        if ecdsa.len() != 33 {
            return Err(WalletError::InvalidEcdsaPublicKey);
        }

        if self.is_ecdsa_set() {
            return Err(WalletError::EcdsaPublicKeyAlreadySet);
        }

        let ecdsa = PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .to_bytes();

        self.ecdsa = Some(ecdsa);

        Ok(())
    }

    pub fn insert_chain(&mut self, chain_type: ChainEnum, chain: Chain) {
        self.chains.insert(chain_type, chain);
    }

    pub fn remove_address(&mut self, chain_type: ChainEnum) -> Result<(), WalletError> {
        if self.chains.remove(&chain_type).is_none() {
            return Err(WalletError::MissingAddress);
        }

        Ok(())
    }
}

use super::{
    chain::Chain,
    ecdsa::EcdsaPublicKey,
    error::LedgerError,
    types::{AddressMap, Balance, ChainEnum, ChainMap, PendingEnum, SendResult},
};
use crate::ledger::chain::ChainTrait;
use b3_utils::{currency::TokenAmount, raw_keccak256, Subaccount};
use bitcoin::secp256k1;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Ledger {
    pub public_key: Option<EcdsaPublicKey>,
    pub subaccount: Subaccount,
    pub chains: ChainMap,
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger {
            public_key: None,
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
            chains,
            subaccount,
            public_key: None,
        }
    }
}

impl Ledger {
    pub fn is_public_key_set(&self) -> bool {
        self.public_key.is_some()
    }

    pub async fn send(
        &self,
        chain_type: &ChainEnum,
        to: String,
        amount: TokenAmount,
    ) -> Result<SendResult, LedgerError> {
        let chain = self.chain(chain_type)?;

        chain.send(to, amount).await
    }

    pub async fn send_mut(
        &mut self,
        chain_type: ChainEnum,
        to: String,
        amount: TokenAmount,
        fee: Option<u64>,
        memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        let chain = self.chain_mut(chain_type)?;

        chain.send_mut(to, amount, fee, memo).await
    }

    pub async fn balance(&self, chain_type: ChainEnum) -> Result<Balance, LedgerError> {
        match self.chains.get(&chain_type) {
            Some(chain) => chain.balance().await,
            None => Err(LedgerError::MissingAddress),
        }
    }

    pub fn address_map(&self) -> AddressMap {
        let mut addresses = AddressMap::new();

        for (chain_type, chain) in &self.chains {
            let address = chain.address();

            addresses.insert(chain_type.clone(), address);
        }

        addresses
    }

    pub fn pendings(&self) -> Vec<PendingEnum> {
        self.chains
            .iter()
            .flat_map(|(_, chain)| chain.pendings())
            .collect()
    }

    pub fn public_key(&self) -> Result<&EcdsaPublicKey, LedgerError> {
        match &self.public_key {
            Some(public_key) => Ok(public_key),
            None => Err(LedgerError::MissingEcdsaPublicKey),
        }
    }

    pub fn eth_public_key(&self) -> Result<secp256k1::PublicKey, LedgerError> {
        let ecdsa = self.public_key()?;

        ecdsa.to_secp256k1_public_key()
    }

    pub fn chain(&self, chains: &ChainEnum) -> Result<&Chain, LedgerError> {
        self.chains
            .get(chains)
            .ok_or_else(|| LedgerError::MissingAddress)
    }

    pub fn chain_mut(&mut self, chains: ChainEnum) -> Result<&mut Chain, LedgerError> {
        self.chains
            .get_mut(&chains)
            .ok_or_else(|| LedgerError::MissingAddress)
    }

    pub fn eth_address(&self) -> Result<String, LedgerError> {
        let public_key = self.eth_public_key()?;

        let pub_key = public_key.serialize_uncompressed();

        let keccak256 = raw_keccak256(&pub_key[1..]);

        let keccak256_hex = keccak256.to_hex_string();

        let address = "0x".to_owned() + &keccak256_hex[24..];

        Ok(address)
    }

    pub async fn new_chain(&self, chain_type: ChainEnum) -> Result<Chain, LedgerError> {
        let subaccount = self.subaccount.to_owned();

        match chain_type {
            ChainEnum::CKBTC(btc_network) => {
                let chain = Chain::new_ckbtc_chain(btc_network, subaccount).await?;

                Ok(chain)
            }
            ChainEnum::ICRC(canister_id) => {
                let chain = Chain::new_icrc_chain(canister_id, subaccount).await?;

                Ok(chain)
            }
            ChainEnum::BTC(btc_network) => {
                let ecdsa = self.public_key()?;

                let btc_chain = Chain::new_btc_chain(btc_network, subaccount, ecdsa.clone())?;

                Ok(btc_chain)
            }
            ChainEnum::EVM(chain_id) => {
                let eth_address = self.eth_address()?;

                let eth_chain = Chain::new_evm_chain(chain_id, eth_address);

                Ok(eth_chain)
            }
            ChainEnum::ICP => {
                let icp_chain = Chain::new_icp_chain(subaccount);

                Ok(icp_chain)
            }
        }
    }

    pub fn set_ecdsa_public_key(&mut self, ecdsa: Vec<u8>) -> Result<(), LedgerError> {
        if self.is_public_key_set() {
            return Err(LedgerError::EcdsaPublicKeyAlreadySet);
        }

        let ecdsa = EcdsaPublicKey::from_slice(&ecdsa)?;

        self.public_key = Some(ecdsa);

        Ok(())
    }

    pub fn insert_chain(&mut self, chain_type: ChainEnum, chain: Chain) {
        self.chains.insert(chain_type, chain);
    }

    pub fn remove_address(&mut self, chain_type: ChainEnum) -> Result<(), LedgerError> {
        if self.chains.remove(&chain_type).is_none() {
            return Err(LedgerError::MissingAddress);
        }

        Ok(())
    }
}

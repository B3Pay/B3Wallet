use super::{
    btc::{btc::BtcChain, network::BtcNetwork},
    chain::Chain,
    ckbtc::ckbtc::CkbtcChain,
    error::LedgerError,
    icp::icp::IcpChain,
    icrc::types::IcrcChain,
    types::{AddressMap, Balance, ChainEnum, ChainMap, EcdsaPublicKey, EvmChain, SendResult},
};
use crate::{
    ledger::chain::ChainTrait,
    types::{PendingReceiveMap, PendingSendMap},
};
use b3_helper_lib::{
    raw_keccak256,
    subaccount::Subaccount,
    types::{BlockIndex, CanisterId},
};
use bitcoin::{secp256k1, Address, PublicKey};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Ledger {
    pub public_key: Option<EcdsaPublicKey>,
    pub subaccount: Subaccount,
    pub chains: ChainMap,
    pub pending_sends: PendingSendMap,
    pub pending_receives: PendingReceiveMap,
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger {
            public_key: None,
            chains: ChainMap::default(),
            subaccount: Subaccount::default(),
            pending_receives: PendingReceiveMap::default(),
            pending_sends: PendingSendMap::default(),
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
            pending_receives: PendingReceiveMap::default(),
            pending_sends: PendingSendMap::default(),
        }
    }
}

impl Ledger {
    pub fn is_public_key_set(&self) -> bool {
        self.public_key.is_some()
    }

    pub async fn send(
        &self,
        chain_type: ChainEnum,
        to: String,
        amount: u64,
    ) -> Result<SendResult, LedgerError> {
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

    pub fn addresses(&self) -> AddressMap {
        let mut addresses = AddressMap::new();

        for (chain_type, chain) in &self.chains {
            let address = chain.address();

            addresses.insert(chain_type.clone(), address);
        }

        addresses
    }

    pub fn pending_receives(&self) -> &PendingReceiveMap {
        &self.pending_receives
    }

    pub fn has_pending(&self, btc_network: BtcNetwork) -> bool {
        self.pending_receives.contains_key(&btc_network)
    }

    pub fn add_pending_receive(&mut self, btc_network: BtcNetwork, txid: String) {
        self.pending_receives.insert(btc_network, txid);
    }

    pub fn remove_pending_receive(&mut self, btc_network: BtcNetwork) {
        self.pending_receives.remove(&btc_network);
    }

    pub fn pending_sends(&self) -> &PendingSendMap {
        &self.pending_sends
    }

    pub fn add_pending_send(&mut self, btc_network: BtcNetwork, block_index: BlockIndex) {
        if let Some(block_indexes) = self.pending_sends.get_mut(&btc_network) {
            block_indexes.push(block_index);
        } else {
            self.pending_sends.insert(btc_network, vec![block_index]);
        }
    }

    pub fn remove_pending_send(&mut self, btc_network: BtcNetwork, block_index: BlockIndex) {
        if let Some(block_indexes) = self.pending_sends.get_mut(&btc_network) {
            block_indexes.retain(|&x| x != block_index);
        }
    }

    pub fn clear_pending_sends(&mut self) {
        self.pending_sends.clear();
    }

    pub fn public_key(&self) -> Result<&EcdsaPublicKey, LedgerError> {
        match &self.public_key {
            Some(public_key) => Ok(public_key),
            None => Err(LedgerError::MissingEcdsaPublicKey),
        }
    }

    pub fn btc_public_key(&self) -> Result<PublicKey, LedgerError> {
        let ecdsa = self.public_key()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|_| LedgerError::InvalidEcdsaPublicKey)?;

        Ok(public_key)
    }

    pub fn eth_public_key(&self) -> Result<secp256k1::PublicKey, LedgerError> {
        let ecdsa = self.btc_public_key()?;

        Ok(ecdsa.inner)
    }

    pub fn chain(&self, chains: ChainEnum) -> Result<&Chain, LedgerError> {
        self.chains
            .get(&chains)
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

    pub fn btc_address(&self, btc_network: BtcNetwork) -> Result<Address, LedgerError> {
        let public_key = self.btc_public_key()?;

        let address = Address::p2pkh(&public_key, btc_network.into());

        Ok(address)
    }

    pub async fn new_chain(&self, chain_type: ChainEnum) -> Result<Chain, LedgerError> {
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

    pub fn icrc(&self, canister_id: CanisterId) -> Option<&IcrcChain> {
        let chain = self.chains.get(&ChainEnum::ICRC(canister_id))?;

        chain.icrc()
    }

    pub fn icrc_mut(&mut self, canister_id: CanisterId) -> Option<&mut IcrcChain> {
        let chain = self.chains.get_mut(&ChainEnum::ICRC(canister_id))?;

        chain.icrc_mut()
    }

    pub fn ckbtc(&self, network: BtcNetwork) -> Option<&CkbtcChain> {
        let chain = self.chains.get(&ChainEnum::CKBTC(network))?;

        chain.ckbtc()
    }

    pub fn ckbtc_mut(&mut self, network: BtcNetwork) -> Option<&mut CkbtcChain> {
        let chain = self.chains.get_mut(&ChainEnum::CKBTC(network))?;

        chain.ckbtc_mut()
    }

    pub fn icp(&self) -> Option<&IcpChain> {
        let chain = self.chains.get(&ChainEnum::ICP)?;

        chain.icp()
    }

    pub fn icp_mut(&mut self) -> Option<&mut IcpChain> {
        let chain = self.chains.get_mut(&ChainEnum::ICP)?;

        chain.icp_mut()
    }

    pub fn evm(&self, chain_id: u64) -> Option<&EvmChain> {
        let chain = self.chains.get(&ChainEnum::EVM(chain_id))?;

        chain.evm()
    }

    pub fn evm_mut(&mut self, chain_id: u64) -> Option<&mut EvmChain> {
        let chain = self.chains.get_mut(&ChainEnum::EVM(chain_id))?;

        chain.evm_mut()
    }

    pub fn btc(&self, network: BtcNetwork) -> Option<&BtcChain> {
        let chain = self.chains.get(&ChainEnum::BTC(network))?;

        chain.btc()
    }

    pub fn btc_mut(&mut self, network: BtcNetwork) -> Option<&mut BtcChain> {
        let chain = self.chains.get_mut(&ChainEnum::BTC(network))?;

        chain.btc_mut()
    }

    pub fn set_ecdsa_public_key(&mut self, ecdsa: Vec<u8>) -> Result<(), LedgerError> {
        if self.is_public_key_set() {
            return Err(LedgerError::EcdsaPublicKeyAlreadySet);
        }

        let ecdsa = PublicKey::from_slice(&ecdsa)
            .map_err(|e| LedgerError::EcdsaPublicKeyError(e.to_string()))?
            .to_bytes();

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

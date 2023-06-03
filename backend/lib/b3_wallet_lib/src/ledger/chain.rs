use super::{
    btc::network::BtcNetwork,
    types::{Balance, Chain, ChainId, ChainTrait, ICRCFee, BTC, EVM, ICP, ICRC},
};
use crate::error::WalletError;
use async_trait::async_trait;
use b3_helper_lib::types::{AccountIdentifier, CanisterId, Subaccount};

impl Chain {
    pub fn new_icrc_chain(canister_id: CanisterId, subaccount: Subaccount) -> Self {
        Chain::ICRC(ICRC::new(canister_id, subaccount, ICRCFee::from(0)))
    }

    pub fn new_btc_chain(btc_network: BtcNetwork, address: String) -> Self {
        Chain::BTC(BTC {
            btc_network,
            address,
        })
    }

    pub fn new_evm_chain(chain_id: ChainId, address: String) -> Self {
        Chain::EVM(EVM { chain_id, address })
    }

    pub fn new_icp_chain(identifier: AccountIdentifier) -> Self {
        Chain::ICP(ICP::new(identifier))
    }

    pub fn address(&self) -> String {
        match self {
            // TODO: implement the address method for ICRC
            Chain::ICRC(icrc) => icrc.canister_id.to_string(),
            Chain::BTC(btc) => btc.address.clone(),
            Chain::EVM(evm) => evm.address.clone(),
            Chain::ICP(icp) => icp.identifier.to_string(),
        }
    }
}

#[async_trait]
impl ChainTrait for BTC {
    async fn balance(&self) -> Result<Balance, WalletError> {
        todo!("implement the async method for BTC...")
    }
}

#[async_trait]
impl ChainTrait for EVM {
    async fn balance(&self) -> Result<Balance, WalletError> {
        todo!("implement the async method for EVM...")
    }
}

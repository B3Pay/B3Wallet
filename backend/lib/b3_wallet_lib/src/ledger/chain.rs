use super::{
    btc::network::BtcNetwork,
    types::{Balance, Chain, ChainId, ChainTrait, ICRCFee, SendResult, BTC, EVM, ICP, ICRC},
};
use crate::error::WalletError;
use async_trait::async_trait;
use b3_helper_lib::types::{CanisterId, Subaccount};

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

    pub fn new_icp_chain(subaccount: Subaccount) -> Self {
        Chain::ICP(ICP::new(subaccount))
    }
}

#[async_trait]
impl ChainTrait for BTC {
    fn address(&self) -> String {
        self.address.clone()
    }

    async fn balance(&self) -> Result<Balance, WalletError> {
        todo!("implement the async method for BTC...")
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, WalletError> {
        todo!("implement the async method for BTC...")
    }
}

#[async_trait]
impl ChainTrait for EVM {
    fn address(&self) -> String {
        self.address.clone()
    }

    async fn balance(&self) -> Result<Balance, WalletError> {
        todo!("implement the async method for EVM...")
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, WalletError> {
        todo!("implement the async method for EVM...")
    }
}

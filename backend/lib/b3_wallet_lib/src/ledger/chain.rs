use super::{
    btc::network::BtcNetwork,
    icrc::{icrc1::ICRC1, types::ICRC},
    types::{Balance, Chain, ChainId, ChainTrait, SendResult, BTC, EVM, ICP},
};
use crate::error::WalletError;
use async_trait::async_trait;
use b3_helper_lib::{error::ErrorTrait, subaccount::Subaccount, types::CanisterId};

impl Chain {
    pub async fn new_icrc_chain(
        canister_id: CanisterId,
        subaccount: Subaccount,
    ) -> Result<Self, WalletError> {
        let icrc1 = ICRC1(canister_id.clone());

        let metadata = icrc1
            .metadata()
            .await
            .map_err(|e| WalletError::ICRC1Error(e.to_string()))?;

        let fee = icrc1
            .fee()
            .await
            .map_err(|e| WalletError::ICRC1Error(e.to_string()))?;

        let chain = Chain::ICRC(ICRC::new(canister_id, subaccount, fee, metadata));

        Ok(chain)
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

    async fn send(&self, _to: String, _amount: u64) -> Result<SendResult, WalletError> {
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

    async fn send(&self, _to: String, _amount: u64) -> Result<SendResult, WalletError> {
        todo!("implement the async method for EVM...")
    }
}

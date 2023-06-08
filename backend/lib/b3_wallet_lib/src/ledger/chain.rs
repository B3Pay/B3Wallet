use super::{
    btc::network::BtcNetwork,
    ckbtc::ckbtc::CKBTC,
    icrc::types::ICRC,
    types::{Balance, ChainId, ChainTrait, SendResult, BTC, EVM, ICP},
};
use crate::error::WalletError;
use async_trait::async_trait;
use b3_helper_lib::{subaccount::Subaccount, types::CanisterId};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[enum_dispatch(ChainTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Chain {
    CKBTC,
    ICRC,
    BTC,
    EVM,
    ICP,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::ICP(ICP::new(Subaccount::default()))
    }
}

impl Chain {
    pub async fn new_icrc_chain(
        canister_id: CanisterId,
        subaccount: Subaccount,
    ) -> Result<Self, WalletError> {
        let chain = Chain::ICRC(ICRC::new(canister_id, subaccount).await?);

        Ok(chain)
    }

    pub async fn new_ckbtc_chain(
        btc_network: BtcNetwork,
        subaccount: Subaccount,
    ) -> Result<Self, WalletError> {
        let chain = Chain::CKBTC(CKBTC::new(btc_network, subaccount).await?);

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

    pub fn icrc(&self) -> Option<&ICRC> {
        match self {
            Chain::ICRC(icrc) => Some(icrc),
            _ => None,
        }
    }

    pub fn icrc_mut(&mut self) -> Option<&mut ICRC> {
        match self {
            Chain::ICRC(icrc) => Some(icrc),
            _ => None,
        }
    }

    pub fn ckbtc(&self) -> Option<&CKBTC> {
        match self {
            Chain::CKBTC(ckbtc) => Some(ckbtc),
            _ => None,
        }
    }

    pub fn ckbtc_mut(&mut self) -> Option<&mut CKBTC> {
        match self {
            Chain::CKBTC(ckbtc) => Some(ckbtc),
            _ => None,
        }
    }

    pub fn btc(&self) -> Option<&BTC> {
        match self {
            Chain::BTC(btc) => Some(btc),
            _ => None,
        }
    }

    pub fn btc_mut(&mut self) -> Option<&mut BTC> {
        match self {
            Chain::BTC(btc) => Some(btc),
            _ => None,
        }
    }

    pub fn evm(&self) -> Option<&EVM> {
        match self {
            Chain::EVM(evm) => Some(evm),
            _ => None,
        }
    }

    pub fn evm_mut(&mut self) -> Option<&mut EVM> {
        match self {
            Chain::EVM(evm) => Some(evm),
            _ => None,
        }
    }

    pub fn icp(&self) -> Option<&ICP> {
        match self {
            Chain::ICP(icp) => Some(icp),
            _ => None,
        }
    }

    pub fn icp_mut(&mut self) -> Option<&mut ICP> {
        match self {
            Chain::ICP(icp) => Some(icp),
            _ => None,
        }
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

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, WalletError> {
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

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, WalletError> {
        todo!("implement the async method for BTC...")
    }
}

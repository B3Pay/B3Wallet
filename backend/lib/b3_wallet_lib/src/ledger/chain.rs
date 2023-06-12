use super::{
    btc::{btc::BtcChain, network::BtcNetwork},
    ckbtc::ckbtc::CkbtcChain,
    error::LedgerError,
    icp::icp::IcpChain,
    icrc::types::IcrcChain,
    types::{Balance, ChainId, EvmChain, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{subaccount::Subaccount, types::CanisterId};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[async_trait]
#[enum_dispatch]
pub trait ChainTrait {
    fn address(&self) -> String;
    async fn balance(&self) -> Result<Balance, LedgerError>;
    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError>;
    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        fee: Option<u64>,
        memo: Option<String>,
    ) -> Result<SendResult, LedgerError>;
}

#[enum_dispatch(ChainTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Chain {
    CkbtcChain,
    IcrcChain,
    BtcChain,
    EvmChain,
    IcpChain,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::IcpChain(IcpChain::new(Subaccount::default()))
    }
}

impl Chain {
    pub async fn new_icrc_chain(
        canister_id: CanisterId,
        subaccount: Subaccount,
    ) -> Result<Self, LedgerError> {
        let icrc = IcrcChain::new(canister_id, subaccount)
            .await
            .map_err(LedgerError::IcrcError)?;
        let chain = Chain::IcrcChain(icrc);

        Ok(chain)
    }

    pub async fn new_ckbtc_chain(
        btc_network: BtcNetwork,
        subaccount: Subaccount,
    ) -> Result<Self, LedgerError> {
        let ckbtc = CkbtcChain::new(btc_network, subaccount)
            .await
            .map_err(LedgerError::CkbtcError)?;
        let chain = Chain::CkbtcChain(ckbtc);

        Ok(chain)
    }

    pub fn new_btc_chain(btc_network: BtcNetwork, address: String) -> Self {
        Chain::BtcChain(BtcChain {
            btc_network,
            address,
        })
    }

    pub fn new_evm_chain(chain_id: ChainId, address: String) -> Self {
        Chain::EvmChain(EvmChain { chain_id, address })
    }

    pub fn new_icp_chain(subaccount: Subaccount) -> Self {
        Chain::IcpChain(IcpChain::new(subaccount))
    }

    pub fn icrc(&self) -> Option<&IcrcChain> {
        match self {
            Chain::IcrcChain(icrc) => Some(icrc),
            _ => None,
        }
    }

    pub fn icrc_mut(&mut self) -> Option<&mut IcrcChain> {
        match self {
            Chain::IcrcChain(icrc) => Some(icrc),
            _ => None,
        }
    }

    pub fn ckbtc(&self) -> Option<&CkbtcChain> {
        match self {
            Chain::CkbtcChain(ckbtc) => Some(ckbtc),
            _ => None,
        }
    }

    pub fn ckbtc_mut(&mut self) -> Option<&mut CkbtcChain> {
        match self {
            Chain::CkbtcChain(ckbtc) => Some(ckbtc),
            _ => None,
        }
    }

    pub fn btc(&self) -> Option<&BtcChain> {
        match self {
            Chain::BtcChain(btc) => Some(btc),
            _ => None,
        }
    }

    pub fn btc_mut(&mut self) -> Option<&mut BtcChain> {
        match self {
            Chain::BtcChain(btc) => Some(btc),
            _ => None,
        }
    }

    pub fn evm(&self) -> Option<&EvmChain> {
        match self {
            Chain::EvmChain(evm) => Some(evm),
            _ => None,
        }
    }

    pub fn evm_mut(&mut self) -> Option<&mut EvmChain> {
        match self {
            Chain::EvmChain(evm) => Some(evm),
            _ => None,
        }
    }

    pub fn icp(&self) -> Option<&IcpChain> {
        match self {
            Chain::IcpChain(icp) => Some(icp),
            _ => None,
        }
    }

    pub fn icp_mut(&mut self) -> Option<&mut IcpChain> {
        match self {
            Chain::IcpChain(icp) => Some(icp),
            _ => None,
        }
    }
}

#[async_trait]
impl ChainTrait for EvmChain {
    fn address(&self) -> String {
        self.address.clone()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn send(&self, _to: String, _amount: u64) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for EVM...")
    }

    async fn send_mut(
        &mut self,
        _to: String,
        _amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        todo!("implement the async method for BTC...")
    }
}

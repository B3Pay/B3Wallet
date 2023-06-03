use async_trait::async_trait;
use b3_helper_lib::types::CanisterId;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use super::{btc::network::BtcNetwork, types::ChainId};

#[async_trait]
#[enum_dispatch]
pub trait ChainTrait {
    // async function
    async fn balance(&self) -> u64;
    async fn transfer(&self, to: String, amount: u64) -> u64;
}

#[enum_dispatch(ChainTrait)]
#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Chains {
    ICRC,
    BTC,
    EVM,
    ICP,
}

#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct ICRC(pub CanisterId);
#[async_trait]
impl ChainTrait for ICRC {
    async fn balance(&self) -> u64 {
        todo!("implement the async method for ICRC...")
    }

    async fn transfer(&self, to: String, amount: u64) -> u64 {
        todo!("implement the async method for ICRC...")
    }
}

#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct BTC(pub BtcNetwork);
#[async_trait]
impl ChainTrait for BTC {
    async fn balance(&self) -> u64 {
        todo!("implement the async method for BTC...")
    }

    async fn transfer(&self, to: String, amount: u64) -> u64 {
        todo!("implement the async method for BTC...")
    }
}

#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct EVM(pub ChainId);
#[async_trait]
impl ChainTrait for EVM {
    async fn balance(&self) -> u64 {
        todo!("implement the async method for EVM...")
    }

    async fn transfer(&self, to: String, amount: u64) -> u64 {
        todo!("implement the async method for EVM...")
    }
}

#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct ICP;
#[async_trait]
impl ChainTrait for ICP {
    async fn balance(&self) -> u64 {
        todo!("implement the async method for ICP...")
    }

    async fn transfer(&self, to: String, amount: u64) -> u64 {
        todo!("implement the async method for ICP...")
    }
}

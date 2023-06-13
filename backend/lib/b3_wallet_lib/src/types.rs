use crate::{
    account::WalletAccount,
    ledger::{btc::network::BtcNetwork, types::AddressMap},
};
use b3_helper_lib::{environment::Environment, types::BlockIndex};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type AccountId = String;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

pub type Metadata = HashMap<String, String>;

pub type PendingReceiveMap = HashMap<BtcNetwork, String>;

pub type PendingSendMap = HashMap<BtcNetwork, Vec<BlockIndex>>;

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletAccountView {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub metadata: Metadata,
    pub addresses: AddressMap,
    pub environment: Environment,
    pub pending_send: PendingSendMap,
    pub pending_receive: PendingReceiveMap,
}

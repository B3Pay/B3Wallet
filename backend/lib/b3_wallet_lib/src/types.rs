use crate::{
    account::WalletAccount,
    ledger::types::{AddressMap, ChainEnum},
};
use b3_helper_lib::environment::Environment;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type AccountId = String;

pub type Metadata = HashMap<String, String>;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

pub type PendingMap = BTreeMap<ChainEnum, String>;

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletAccountView {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub metadata: Metadata,
    pub environment: Environment,
    pub addresses: AddressMap,
    pub pendings: PendingMap,
}

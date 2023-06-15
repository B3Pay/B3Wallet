use crate::{
    account::WalletAccount,
    ledger::types::{AddressMap, PendingMap},
};
use b3_helper_lib::environment::Environment;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type AccountId = String;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

pub type Metadata = HashMap<String, String>;

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletAccountView {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub metadata: Metadata,
    pub addresses: AddressMap,
    pub environment: Environment,
    pub pendings: PendingMap,
}

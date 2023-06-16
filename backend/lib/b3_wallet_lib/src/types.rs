use crate::{
    account::WalletAccount,
    ledger::types::{AddressMap, Pendings},
};
use b3_helper_lib::{environment::Environment, types::Metadata};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::BTreeMap;

pub type AccountId = String;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletAccountView {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub metadata: Metadata,
    pub addresses: AddressMap,
    pub environment: Environment,
    pub pendings: Pendings,
}

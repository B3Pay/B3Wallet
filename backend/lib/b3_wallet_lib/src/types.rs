use crate::{
    account::WalletAccount,
    ledger::types::{AddressMap, Pendings},
};
use b3_utils::{types::Metadata, Environment};
use candid::CandidType;
use serde::Deserialize;

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

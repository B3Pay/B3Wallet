use crate::{account::WalletAccount, ledger::types::ChainMap};
use b3_helper_lib::types::Environment;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::{BTreeMap, HashMap};

pub type AccountId = String;

pub type Metadata = HashMap<String, String>;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

#[derive(CandidType, Clone, Deserialize)]
pub struct WalletAccountView {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub metadata: Metadata,
    pub environment: Environment,
    pub addresses: ChainMap,
}

use crate::{account::WalletAccount, ledger::types::AddressMap};
use b3_helper_lib::types::{B3Path, Environment};
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
    pub addresses: AddressMap,
}

pub trait PathTrait {
    fn retrieve(&self) -> Option<&'static [u8]>;
}

impl PathTrait for B3Path {
    fn retrieve(&self) -> Option<&'static [u8]> {
        match self.0.as_str() {
            "/index.html" | "/" => Some(include_bytes!("../../../../out/index.html")),
            // "/favicon.ico" => Some(include_bytes!("../../../../out/favicon.ico")),
            // "/index.js" => Some(include_bytes!("../../../../out/index.js")),
            _ => None,
        }
    }
}

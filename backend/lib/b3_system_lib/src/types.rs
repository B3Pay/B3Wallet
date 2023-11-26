use b3_utils::{
    ledger::types::{Bug, WalletVersion},
    memory::types::{Bound, Storable},
    types::{ControllerId, UserId},
    wasm::{WasmHash, WasmSize},
    NanoTimeStamp,
};
use candid::CandidType;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::{user::UserState, wallet::WalletCanister};

pub type ReleaseVersion = String;

pub type UserStates = Vec<UserState>;
pub type Controllers = Vec<ControllerId>;

pub type Features = Vec<String>;
pub type Users = Vec<UserId>;

pub type Canisters = Vec<WalletCanister>;

#[derive(CandidType)]
pub struct LoadRelease {
    pub total: usize,
    pub chunks: usize,
    pub version: WalletVersion,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Release {
    pub name: String,
    pub date: NanoTimeStamp,
    pub size: WasmSize,
    pub hash: WasmHash,
    pub version: WalletVersion,
    pub deprecated: bool,
    pub features: Features,
}

impl Storable for Release {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ReleaseArgs {
    pub size: usize,
    pub name: String,
    pub version: ReleaseVersion,
    pub features: Features,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct WalletBugs(Vec<Bug>);

impl Storable for WalletBugs {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

impl WalletBugs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, bug: Bug) {
        self.0.push(bug);
    }

    pub fn drain(&mut self) -> Vec<Bug> {
        self.0.drain(..).collect()
    }
}

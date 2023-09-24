use std::borrow::Cow;

use b3_utils::{
    memory::types::{Bound, Storable},
    types::CanisterId,
    wasm::WasmHash,
};
use candid::{CandidType, Decode, Encode};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Clone)]
pub struct Bug {
    pub name: String,
    pub description: String,
    pub logs: Vec<String>,
    pub version: String,
    pub hash: WasmHash,
    pub canister_id: CanisterId,
}

impl<'de> Storable for Bug {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes<'a>(bytes: Cow<'a, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

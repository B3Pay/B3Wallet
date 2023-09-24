use crate::{types::CanisterId, NanoTimeStamp};
use candid::{CandidType, Decode, Encode};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: usize,
    pub canister_id: CanisterId,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Bug {
    pub name: String,
    pub description: String,
    pub logs: Vec<String>,
    pub version: String,
    pub canister_id: CanisterId,
}

impl Bug {
    const MAX_STRING_LENGTH: u32 = 256; // Maximum characters in a String
    const MAX_LOG_ENTRIES: u32 = 10;

    // Calculating the maximum possible size for the `Bug` struct.
    pub const MAX_SIZE: u32 = 24 + // Size of Vec<String> itself (logs)
        (Self::MAX_LOG_ENTRIES * (24 + Self::MAX_STRING_LENGTH)) + // Maximum size of logs
        (3 * (24 + Self::MAX_STRING_LENGTH)) + // Maximum size for name, description, and version
        30; // size of CanisterId
}

#[cfg(feature = "stable_memory")]
use crate::memory::types::{Bound, Storable};

#[cfg(feature = "stable_memory")]
impl Storable for Bug {
    const BOUND: Bound = Bound::Bounded {
        max_size: Self::MAX_SIZE,
        is_fixed_size: false, // Size is not fixed because of the Vec and Strings
    };

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

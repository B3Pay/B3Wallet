use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type ControllerId = Principal;
pub type ControllerIds = Vec<ControllerId>;

pub type CanisterId = Principal;
pub type SignerId = Principal;

pub type RequestId = usize;
pub type Deadline = u64;

pub type Cycles = u128;

pub type Metadata = HashMap<String, String>;

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferMemo(pub u64);

pub type TransferBlockIndex = u64;

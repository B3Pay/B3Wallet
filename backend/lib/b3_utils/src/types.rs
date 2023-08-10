use std::collections::HashMap;

mod icp;
mod system;
mod wallet;

pub use icp::*;
pub use system::*;
pub use wallet::*;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type ControllerId = Principal;
pub type ControllerIds = Vec<ControllerId>;

pub type CanisterId = Principal;
pub type UserId = Principal;

pub type OperationId = usize;
pub type Deadline = u64;

pub type Cycles = u128;

pub type Metadata = HashMap<String, String>;

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferMemo(pub u64);

pub type TransferBlockIndex = u64;

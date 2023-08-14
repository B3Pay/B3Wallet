use crate::nonce::Nonce;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod icp;
pub use icp::*;

mod system;
pub use system::*;

mod wallet;
pub use wallet::*;

pub type ControllerId = Principal;
pub type ControllerIds = Vec<ControllerId>;

pub type CanisterId = Principal;
pub type UserId = Principal;

pub type RoleId = Nonce;

pub type OperationId = u64;
pub type Deadline = u64;

pub type Cycles = u128;

pub type Metadata = HashMap<String, String>;

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferMemo(pub u64);

pub type TransferBlockIndex = u64;

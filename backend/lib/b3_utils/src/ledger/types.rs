mod icp;
use candid::CandidType;
use serde::{Deserialize, Serialize};

pub use icp::*;

mod system;
pub use system::*;

mod wallet;
pub use wallet::*;

pub type Cycles = u128;

#[derive(CandidType, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferMemo(pub u64);

pub type TransferBlockIndex = u64;

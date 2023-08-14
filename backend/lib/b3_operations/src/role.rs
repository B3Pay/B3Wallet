use crate::operation::Operation;

use b3_utils::NanoTimeStamp;
use candid::{CandidType, Deserialize};

mod state;
pub use state::*;

#[derive(CandidType, Deserialize, PartialEq, Debug, Clone)]
pub struct OperationAccess {
    operation: Operation,
    valid_until: Option<NanoTimeStamp>,
}

#[derive(CandidType, Deserialize, PartialEq, Debug, Clone)]
pub enum AccessLevel {
    FullAccess,
    ReadOnly,
    Limited(Vec<OperationAccess>),
}

#[derive(CandidType, Deserialize, PartialEq, Debug, Clone)]
pub struct Role {
    name: String,
    access_level: AccessLevel,
}

impl Default for Role {
    fn default() -> Self {
        Role {
            name: "default".to_string(),
            access_level: AccessLevel::FullAccess,
        }
    }
}

impl Role {
    pub fn new(name: String, access_level: AccessLevel) -> Self {
        Role { name, access_level }
    }

    pub fn has_operation(&self, operation: Operation) -> bool {
        match &self.access_level {
            AccessLevel::FullAccess => true,
            AccessLevel::ReadOnly => todo!("ReadOnly"),
            AccessLevel::Limited(operations) => operations.iter().any(|op_access| {
                if op_access.operation == operation {
                    if let Some(valid_until) = &op_access.valid_until {
                        if valid_until.has_passed() {
                            return false;
                        }
                    }
                    return true;
                }
                false
            }),
        }
    }
}

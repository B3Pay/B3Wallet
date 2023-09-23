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
    Canister,
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn access_level(&self) -> &AccessLevel {
        &self.access_level
    }

    pub fn is_canister_or_admin(&self) -> bool {
        self.is_canister() || self.is_admin()
    }

    pub fn is_canister(&self) -> bool {
        self.access_level == AccessLevel::Canister
    }

    pub fn is_admin(&self) -> bool {
        self.access_level == AccessLevel::FullAccess
    }

    pub fn is_user(&self) -> bool {
        match &self.access_level {
            AccessLevel::FullAccess => true, // FullAccess is considered a user
            AccessLevel::ReadOnly => false,  // ReadOnly is not considered a user
            AccessLevel::Canister => false,  // Canister is not considered a user
            AccessLevel::Limited(operations) => !operations.is_empty(), // User if at least one operation exists
        }
    }

    pub fn have_access_level(&self, access_level: &AccessLevel) -> bool {
        match access_level {
            AccessLevel::FullAccess => self.is_admin(),
            AccessLevel::ReadOnly => todo!("ReadOnly"),
            AccessLevel::Canister => self.is_canister(),
            AccessLevel::Limited(operations) => {
                if self.is_admin() {
                    return true;
                }

                operations.iter().any(|op_access| {
                    if let Some(valid_until) = &op_access.valid_until {
                        if valid_until.has_passed() {
                            return false;
                        }
                    }
                    self.has_operation(&op_access.operation)
                })
            }
        }
    }

    pub fn has_operation(&self, operation: &Operation) -> bool {
        match &self.access_level {
            AccessLevel::FullAccess => true,
            AccessLevel::ReadOnly => todo!("ReadOnly"),
            AccessLevel::Canister => todo!("Canister"),
            AccessLevel::Limited(operations) => operations.iter().any(|op_access| {
                if &op_access.operation == operation {
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

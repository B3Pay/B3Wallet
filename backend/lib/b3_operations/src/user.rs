use b3_utils::types::Metadata;
use candid::{CandidType, Deserialize};

pub mod state;

use crate::{
    operation::Operation,
    role::{AccessLevel, Role},
};

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub role: Role,
    pub name: String,
    pub metadata: Metadata,
    pub expires_at: Option<u64>,
}

impl Default for User {
    fn default() -> Self {
        User {
            role: Role::default(),
            name: "".to_string(),
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl From<Role> for User {
    fn from(role: Role) -> Self {
        User {
            role,
            name: "".to_string(),
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl User {
    pub fn new(role: Role, name: String, expires_at: Option<u64>) -> Self {
        User {
            role,
            name,
            expires_at,
            metadata: Metadata::default(),
        }
    }

    pub fn is_canister_or_admin(&self) -> bool {
        self.role.is_canister_or_admin()
    }

    pub fn is_canister(&self) -> bool {
        self.role.is_canister()
    }

    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    pub fn is_user(&self) -> bool {
        self.role.is_user()
    }

    pub fn have_access_level(&self, access_level: &AccessLevel) -> bool {
        self.role.access_level() == access_level
    }

    pub fn can_operate(&self, operation: &Operation) -> bool {
        self.role.has_operation(operation)
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }
}

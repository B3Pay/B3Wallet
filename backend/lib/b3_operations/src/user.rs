use b3_utils::types::Metadata;
use candid::{CandidType, Deserialize};

pub mod state;

use crate::{operation::Operation, role::Role};

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

    pub fn can_operate(&self, operation: Operation) -> bool {
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

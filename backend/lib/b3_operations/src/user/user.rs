use b3_utils::types::Metadata;
use candid::{CandidType, Deserialize};

use crate::operation::Operation;

use super::role::UserRole;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserState {
    pub role: UserRole,
    pub name: String,
    pub metadata: Metadata,
    pub expires_at: Option<u64>,
}

impl Default for UserState {
    fn default() -> Self {
        UserState {
            role: UserRole::default(),
            name: "".to_string(),
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl From<UserRole> for UserState {
    fn from(role: UserRole) -> Self {
        UserState {
            role,
            name: "".to_string(),
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl UserState {
    pub fn new(role: UserRole, name: String, expires_at: Option<u64>) -> Self {
        UserState {
            role,
            name,
            expires_at,
            metadata: Metadata::default(),
        }
    }

    pub fn can(&self, operation: Operation) -> bool {
        self.role.has_operation(operation)
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_role(&mut self, role: UserRole) {
        self.role = role;
    }

    pub fn has_role(&self, role: UserRole) -> bool {
        role == self.role
    }
}

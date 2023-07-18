use b3_helper_lib::types::Metadata;
use candid::{CandidType, Deserialize};

use super::roles::Roles;

#[derive(CandidType, Deserialize, Clone)]
pub struct Signer {
    pub role: Roles,
    pub name: String,
    pub metadata: Metadata,
    pub threshold: Option<u8>,
    pub expires_at: Option<u64>,
}

impl Default for Signer {
    fn default() -> Self {
        Signer {
            role: Roles::Admin,
            name: "".to_string(),
            threshold: None,
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl From<Roles> for Signer {
    fn from(role: Roles) -> Self {
        Signer {
            role,
            name: "".to_string(),
            threshold: None,
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl Signer {
    pub fn new(role: Roles, name: String, expires_at: Option<u64>) -> Self {
        Signer {
            role,
            name,
            expires_at,
            threshold: None,
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

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_role(&mut self, role: Roles) {
        self.role = role;
    }

    pub fn has_role(&self, role: Roles) -> bool {
        if self.is_admin() {
            return true;
        }

        role == self.role
    }
}

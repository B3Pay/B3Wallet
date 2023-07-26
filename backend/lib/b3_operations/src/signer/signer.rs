use b3_utils::types::Metadata;
use candid::{CandidType, Deserialize};

use super::roles::SignerRoles;

#[derive(CandidType, Deserialize, Clone)]
pub struct Signer {
    pub role: SignerRoles,
    pub name: String,
    pub metadata: Metadata,
    pub threshold: Option<u8>,
    pub expires_at: Option<u64>,
}

impl Default for Signer {
    fn default() -> Self {
        Signer {
            role: SignerRoles::Admin,
            name: "".to_string(),
            threshold: None,
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl From<SignerRoles> for Signer {
    fn from(role: SignerRoles) -> Self {
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
    pub fn new(role: SignerRoles, name: String, expires_at: Option<u64>) -> Self {
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

    pub fn set_role(&mut self, role: SignerRoles) {
        self.role = role;
    }

    pub fn has_role(&self, role: SignerRoles) -> bool {
        if self.is_admin() {
            return true;
        }

        role == self.role
    }
}

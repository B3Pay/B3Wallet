use std::fmt;

use b3_helper::types::Metadata;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, PartialEq, Copy, Clone)]
pub enum Roles {
    Canister,
    Operator,
    Owner,
    Admin,
    User,
}

impl Default for Roles {
    fn default() -> Self {
        Roles::User
    }
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Roles::Canister => write!(f, "Canister"),
            Roles::Operator => write!(f, "Operator"),
            Roles::Owner => write!(f, "Owner"),
            Roles::Admin => write!(f, "Admin"),
            Roles::User => write!(f, "User"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Signer {
    pub role: Roles,
    pub name: Option<String>,
    pub metadata: Metadata,
    pub expires_at: Option<u64>,
}

impl Default for Signer {
    fn default() -> Self {
        Signer {
            role: Roles::User,
            name: None,
            metadata: Metadata::default(),
            expires_at: None,
        }
    }
}

impl From<Roles> for Signer {
    fn from(role: Roles) -> Self {
        Signer {
            role,
            name: None,
            expires_at: None,
            metadata: Metadata::default(),
        }
    }
}

impl Signer {
    pub fn new(role: Roles, name: Option<String>) -> Self {
        Signer {
            role,
            name,
            expires_at: None,
            metadata: Metadata::default(),
        }
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_name(&mut self, name: Option<String>) {
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

    pub fn is_canister_or_admin(&self) -> bool {
        self.is_canister() || self.is_admin()
    }

    pub fn is_canister(&self) -> bool {
        self.role == Roles::Canister
    }

    pub fn is_owner(&self) -> bool {
        self.role == Roles::Owner
    }

    pub fn is_admin(&self) -> bool {
        self.role == Roles::Admin
    }

    pub fn is_user(&self) -> bool {
        self.role == Roles::User
    }
}

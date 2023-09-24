use candid::{CandidType, Deserialize};
use std::fmt;

#[derive(CandidType, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum SignerRoles {
    Canister,
    Admin,
    User,
}

impl Default for SignerRoles {
    fn default() -> Self {
        SignerRoles::User
    }
}

impl fmt::Display for SignerRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignerRoles::Canister => write!(f, "Canister"),
            SignerRoles::Admin => write!(f, "Admin"),
            SignerRoles::User => write!(f, "User"),
        }
    }
}

impl SignerRoles {
    pub fn is_canister_or_admin(&self) -> bool {
        self.is_canister() || self.is_admin()
    }

    pub fn is_canister(&self) -> bool {
        self == &SignerRoles::Canister
    }

    pub fn is_admin(&self) -> bool {
        self == &SignerRoles::Admin
    }

    pub fn is_user(&self) -> bool {
        if self.is_admin() {
            return true;
        }

        self == &SignerRoles::User
    }

    pub fn get_num_signers(&self) -> usize {
        match self {
            SignerRoles::Canister => 1,
            SignerRoles::Admin => 1,
            SignerRoles::User => 1,
        }
    }
}

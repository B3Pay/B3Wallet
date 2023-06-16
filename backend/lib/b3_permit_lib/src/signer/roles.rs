use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::fmt;

#[derive(CandidType, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Roles {
    Threshold,
    Canister,
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
            Roles::Threshold => write!(f, "Threshold"),
            Roles::Canister => write!(f, "Canister"),
            Roles::Admin => write!(f, "Admin"),
            Roles::User => write!(f, "User"),
        }
    }
}

impl Roles {
    pub fn is_canister_or_admin(&self) -> bool {
        self.is_canister() || self.is_admin()
    }

    pub fn is_canister(&self) -> bool {
        self == &Roles::Canister
    }

    pub fn is_admin(&self) -> bool {
        self == &Roles::Admin
    }

    pub fn is_user(&self) -> bool {
        if self.is_admin() {
            return true;
        }

        self == &Roles::User
    }

    pub fn get_num_signers(&self) -> usize {
        match self {
            Roles::Threshold => 0,
            Roles::Canister => 1,
            Roles::Admin => 1,
            Roles::User => 1,
        }
    }
}

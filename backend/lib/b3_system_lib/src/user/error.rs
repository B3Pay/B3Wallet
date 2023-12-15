use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum UserSystemError {
    UserAlreadyExists,
    CreateCanisterError(String),
    ValidateUserError(String),
    WalletCanisterNotFound,
    RateLimitExceeded,
    UserNotFound,
    InvalidUser,
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for UserSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserSystemError::InvalidUser => write!(f, "Invalid user!"),
            UserSystemError::ValidateUserError(e) => write!(f, "Validate user error: {}", e),
            UserSystemError::UserAlreadyExists => write!(f, "User already exists!"),
            UserSystemError::UserNotFound => write!(f, "User not found!"),
            UserSystemError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            UserSystemError::RateLimitExceeded => write!(f, "Rate limit exceeded!"),
            UserSystemError::WalletCanisterNotFound => write!(f, "Wallet canister not found!"),
        }
    }
}

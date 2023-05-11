use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub enum SharedError {
    CanisterStatusError(String),
    InvalidAccountIdentifier,
}

pub trait TrapError {
    fn to_string(self) -> String;
}

impl TrapError for SharedError {
    fn to_string(self) -> String {
        match self {
            SharedError::CanisterStatusError(e) => e.to_string(),
            SharedError::InvalidAccountIdentifier => "Invalid account identifier".to_string(),
        }
    }
}

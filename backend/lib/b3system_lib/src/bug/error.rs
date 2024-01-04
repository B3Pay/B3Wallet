use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum BugSystemError {
    BugsNotFound,
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for BugSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BugSystemError::BugsNotFound => write!(f, "Bugs not found!"),
        }
    }
}

use candid::CandidType;
use serde::{Deserialize, Serialize};
mod test;

#[derive(
    CandidType,
    Default,
    Hash,
    Serialize,
    Deserialize,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Clone,
)]
pub struct Nonce(pub u64);

impl Nonce {
    pub fn new(from: Option<u64>) -> Self {
        Self(from.unwrap_or(0))
    }

    pub fn current(&self) -> Nonce {
        self.clone()
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// increment the counter and return the new value
    pub fn next(&mut self) -> Nonce {
        self.increment();

        self.current()
    }
}

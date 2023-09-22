use candid::CandidType;
use serde::{Deserialize, Serialize};
mod test;

pub mod traits;

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

    pub fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Nonce(u64::from_le_bytes(bytes))
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    pub fn current(&self) -> Nonce {
        self.clone()
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn add_64(&self, other: u64) -> Self {
        Self(self.0 + other)
    }

    pub fn sub_64(&self, other: u64) -> Self {
        Self(self.0 - other)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// increment the counter and return the new value
    pub fn next(&mut self) -> Self {
        self.increment();

        self.current()
    }

    /// increment the counter and return the new value
    pub fn next_get(&mut self) -> u64 {
        self.increment();

        self.get()
    }
}

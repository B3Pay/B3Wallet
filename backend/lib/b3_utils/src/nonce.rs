use candid::CandidType;
use serde::{Deserialize, Serialize};
mod test;

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Nonce(usize);

impl Nonce {
    pub fn new(from: Option<usize>) -> Self {
        Self(from.unwrap_or(0))
    }

    pub fn current(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// increment the counter and return the new value
    pub fn next(&mut self) -> usize {
        self.increment();

        self.0
    }
}

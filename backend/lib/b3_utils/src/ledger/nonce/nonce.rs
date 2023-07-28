use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Nonce(usize);

impl Nonce {
    pub fn new() -> Self {
        Nonce(0)
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

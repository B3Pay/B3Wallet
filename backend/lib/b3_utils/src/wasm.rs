use candid::CandidType;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

mod test;

mod store;
mod types;
mod utils;

pub use store::*;
pub use types::*;
pub use utils::*;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Wasm(pub ByteBuf);

impl Default for Wasm {
    fn default() -> Self {
        Self(ByteBuf::new())
    }
}

impl Wasm {
    pub fn load(&mut self, blob: &Blob) -> WasmSize {
        self.0.extend(blob);

        self.0.len()
    }

    pub fn unload(&mut self) -> WasmSize {
        self.0.clear();

        self.0.len()
    }

    pub fn len(&self) -> WasmSize {
        self.0.len()
    }

    pub fn get(&self) -> Blob {
        self.0.to_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn is_loading(&self, size: usize) -> bool {
        self.0.is_empty() || self.0.len() < size
    }

    pub fn is_loaded(&self, size: usize) -> bool {
        self.0.len() == size
    }

    pub fn generate_hash(&self) -> WasmHash {
        if self.0.is_empty() {
            return WasmHash::default();
        }

        sha256_wasm_hash(&self.0)
    }

    pub fn generate_hash_string(&self) -> String {
        if self.0.is_empty() {
            return String::default();
        }

        sha256_wasm_hash_string(&self.0)
    }

    pub fn is_hashed(&self, hash: &WasmHash) -> bool {
        self.generate_hash() == *hash
    }
}

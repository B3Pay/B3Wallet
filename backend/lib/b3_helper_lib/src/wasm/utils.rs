use easy_hasher::easy_hasher;

use super::{Wasm, WASM};
use crate::types::WasmHash;

pub fn sha256_wasm_hash(data: &[u8]) -> WasmHash {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    let bytes = hash.to_vec();

    let mut wasm_hash: WasmHash = [0; 32];
    wasm_hash.copy_from_slice(&bytes[0..32]);
    wasm_hash
}

pub fn sha256_wasm_hash_string(data: &[u8]) -> String {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    hash.to_hex_string()
}

/// Get wasm.
pub fn with_wasm<T, F>(callback: F) -> T
where
    F: FnOnce(&Wasm) -> T,
{
    WASM.with(|wasm| {
        let wasm = wasm.borrow();

        callback(&wasm)
    })
}

/// Get wasm mutably.
pub fn with_wasm_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut Wasm) -> T,
{
    WASM.with(|wasm| {
        let mut wasm = wasm.borrow_mut();

        callback(&mut wasm)
    })
}

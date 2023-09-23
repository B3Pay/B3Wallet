use crate::permit::{caller_is_admin, caller_is_canister_or_admin, caller_is_signer};
use b3_utils::{
    wasm::{with_wasm, with_wasm_mut},
    wasm::{WasmDetails, WasmHash, WasmSize},
};
use ic_cdk::{query, update};

#[query(guard = "caller_is_canister_or_admin")]
fn wasm_details() -> WasmDetails {
    with_wasm(|w| {
        let hash = w.generate_hash();
        let size = w.len();

        WasmDetails { hash, size }
    })
}

#[query(guard = "caller_is_signer")]
fn wasm_hash_string() -> String {
    with_wasm(|w| w.generate_hash_string())
}

#[query(guard = "caller_is_signer")]
fn wasm_hash() -> WasmHash {
    with_wasm(|w| w.generate_hash())
}

#[update(guard = "caller_is_canister_or_admin")]
fn load_wasm(blob: Vec<u8>) -> WasmSize {
    with_wasm_mut(|w| w.load(&blob))
}

#[update(guard = "caller_is_admin")]
fn unload_wasm() -> WasmSize {
    with_wasm_mut(|w| w.unload())
}

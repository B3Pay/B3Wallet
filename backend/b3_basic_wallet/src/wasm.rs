use b3_utils::{
    owner::caller_is_owner,
    wasm::{with_wasm, with_wasm_mut},
    wasm::{WasmDetails, WasmHash, WasmSize},
};
use candid::candid_method;
use ic_cdk::{query, update};

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
fn wasm_details() -> WasmDetails {
    with_wasm(|w| {
        let hash = w.generate_hash();
        let size = w.len();

        WasmDetails { hash, size }
    })
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
fn wasm_hash_string() -> String {
    with_wasm(|w| w.generate_hash_string())
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
fn wasm_hash() -> WasmHash {
    with_wasm(|w| w.generate_hash())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn load_wasm(blob: Vec<u8>) -> WasmSize {
    with_wasm_mut(|w| w.load(&blob))
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn unload_wasm() -> WasmSize {
    with_wasm_mut(|w| w.unload())
}

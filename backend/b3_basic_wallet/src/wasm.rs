use b3_helper_lib::{
    owner::caller_is_owner,
    types::{WasmDetails, WasmHash, WasmSize},
    wasm::{with_wasm, with_wasm_mut},
};
use ic_cdk::{export::candid::candid_method, query, update};

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

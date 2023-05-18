use crate::signer::caller_is_signer;
use b3_helper::types::{WasmHash, WasmSize};
use b3_wallet_lib::store::{with_state_mut, with_wasm, with_wasm_mut};
use ic_cdk::{export::candid::candid_method, query, update};

#[candid_method(query)]
#[query]
fn wasm_hash() -> WasmHash {
    with_wasm(|w| w.generate_hash())
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
fn load_wasm(blob: Vec<u8>) -> WasmSize {
    with_wasm_mut(|w| w.load(&blob))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
fn unload_wasm() -> WasmSize {
    with_wasm_mut(|w| w.unload())
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn reset_wallet() {
    with_state_mut(|s| s.reset());
}

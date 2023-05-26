use crate::signer::{caller_is_admin, caller_is_canister_or_admin};
use b3_helper_lib::{
    types::{WasmHash, WasmSize},
    wasm::with_wasm_mut,
};
use b3_wallet_lib::store::with_wallet_mut;
use ic_cdk::{export::candid::candid_method, query, update};

#[candid_method(query)]
#[query]
fn wasm_hash() -> WasmHash {
    // with_wasm(|w| w.generate_hash())
    WasmHash::default()
}

#[candid_method(update)]
#[update(guard = "caller_is_canister_or_admin")]
fn load_wasm(blob: Vec<u8>) -> WasmSize {
    with_wasm_mut(|w| w.load(&blob))
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
fn unload_wasm() -> WasmSize {
    with_wasm_mut(|w| w.unload())
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub async fn reset_wallet() {
    with_wallet_mut(|s| s.reset());
}

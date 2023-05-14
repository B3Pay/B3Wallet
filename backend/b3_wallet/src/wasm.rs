use crate::guard::caller_is_signer;
use b3_helper::types::{WasmHash, WasmSize};
use b3_wallet_lib::{
    store::{with_wasm, with_wasm_mut},
    wasm::WasmTrait,
};
use ic_cdk::{
    api::management_canister::main::install_code, export::candid::candid_method, query, update,
};

#[candid_method(query)]
#[query]
fn wasm_hash() -> WasmHash {
    with_wasm(|w| w.generate_hash())
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn upgrade_canister() {
    let args = with_wasm(|w| w.upgrade_args());

    install_code(args).await.unwrap();
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn reintall_canister() {
    let args = with_wasm(|w| w.reintall_args());

    install_code(args).await.unwrap();
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

use b3_helper_lib::{
    owner::caller_is_owner,
    revert,
    types::{WasmHash, WasmSize},
    wasm::{with_wasm, with_wasm_mut},
};
use b3_wallet_lib::error::WalletError;
use ic_cdk::{
    api::management_canister::main::{install_code, CanisterInstallMode, InstallCodeArgument},
    export::candid::candid_method,
    query, update,
};

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

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn upgrage_wallet() {
    let canister_id = ic_cdk::id();
    let wasm_module = with_wasm(|w| {
        if w.is_empty() {
            return revert(WalletError::WasmNotLoaded);
        }
        w.get()
    });

    let args = InstallCodeArgument {
        canister_id,
        wasm_module,
        arg: Vec::new(),
        mode: CanisterInstallMode::Upgrade,
    };

    install_code(args).await.unwrap();
}

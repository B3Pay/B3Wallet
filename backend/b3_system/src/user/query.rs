use crate::guards::caller_is_controller;
use b3_helper::{
    b3_revert,
    types::{SignerCanister, UserId, Version, WasmHash},
};
use b3_system_lib::{
    store::{with_signer_canister, with_state},
    types::SignerCanisters,
};
use ic_cdk::{caller, export::candid::candid_method, query};

#[candid_method(query)]
#[query]
pub fn get_canister() -> SignerCanister {
    let user = caller();

    with_signer_canister(&user, |c| c.clone()).unwrap_or_else(|e| b3_revert(e))
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_user_ids() -> Vec<UserId> {
    with_state(|s| s.user_ids())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_signer_canisters() -> SignerCanisters {
    with_state(|s| s.signer_canisters())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_version(user: UserId) -> Version {
    let signer = with_signer_canister(&user, |c| c.clone()).unwrap_or_else(|e| b3_revert(e));

    signer.version().await.unwrap_or_else(|e| b3_revert(e))
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_wasmhash(user: UserId) -> WasmHash {
    let signer = with_signer_canister(&user, |c| c.clone()).unwrap_or_else(|e| b3_revert(e));

    signer.wasm_hash().await.unwrap_or_else(|e| b3_revert(e))
}

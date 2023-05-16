mod guard;
mod release;
mod status;
mod test;
mod user;

use b3_system_lib::{
    store::{with_state, with_state_mut, with_wasm_map, with_wasm_map_mut},
    types::{State, WasmMap},
};
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, init, post_upgrade, pre_upgrade};

#[init]
#[candid_method(init)]
pub fn init() {
    // TODO: Remove this function and get owner from argument.
    let owner = Principal::anonymous();
    let manager = caller();

    with_state_mut(|s| {
        s.add_controller(owner);
        s.add_controller(manager);
    });
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let wasm = with_wasm_map(|m| m.clone());

    let state = with_state(|s| s.clone());

    ic_cdk::storage::stable_save((state, wasm)).unwrap();
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev, w_prev): (State, WasmMap) = ic_cdk::storage::stable_restore().unwrap();
    with_state_mut(|s| {
        *s = s_prev;
    });

    with_wasm_map_mut(|m| {
        *m = w_prev;
    });
}

#[cfg(test)]
mod tests {
    use b3_helper::types::*;
    use b3_system_lib::error::SystemError;
    use b3_system_lib::types::*;
    use ic_cdk::export::candid::export_service;

    #[test]
    fn generate_candid() {
        use std::io::Write;

        export_service!();

        let candid = __export_service();

        let mut file = std::fs::File::create("./b3_system.did").unwrap();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}

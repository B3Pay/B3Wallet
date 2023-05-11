mod account;
mod guards;
mod status;
mod wasm;

use b3_shared::types::{UserControlArgs, UserId};
use b3_signer_lib::{
    state::State,
    store::{with_state, with_state_mut},
};
use ic_cdk::{api::call::arg_data, export::candid::candid_method, init, post_upgrade, pre_upgrade};

use guards::OWNER;
use wasm::WASM;

#[init]
#[candid_method(init)]
pub fn init() {
    let (call_arg,) = arg_data::<(Option<UserControlArgs>,)>();

    let owner = match call_arg {
        Some(args) => args.owner,
        None => ic_cdk::caller(),
    };

    with_state_mut(|s| {
        s.init();
    });

    OWNER.with(|o| {
        *o.borrow_mut() = owner;
    });
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let owner = OWNER.with(|o| o.borrow().clone());
    with_state(|s| {
        ic_cdk::storage::stable_save((s, owner)).unwrap();
    });

    WASM.with(|s| s.borrow_mut().reset());
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev, owner_prev): (State, UserId) = ic_cdk::storage::stable_restore().unwrap();
    with_state_mut(|s| {
        *s = s_prev;
    });

    OWNER.with(|o| {
        *o.borrow_mut() = owner_prev;
    });
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmData;
    use b3_shared::types::*;

    use b3_signer_lib::{
        account::SignerAccount,
        ledger::types::*,
        ledger::{config::Environment, network::Network},
        request::EvmSignRequest,
        signed::SignedTransaction,
        state::State,
        types::{AccountsStatus, CanisterAllowances, SignerAllowanceArgs},
    };
    use ic_cdk::export::candid::export_service;

    #[test]
    fn generate_candid() {
        use std::io::Write;

        let mut file = std::fs::File::create("./b3_signer.did").unwrap();

        export_service!();

        let candid = __export_service();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}

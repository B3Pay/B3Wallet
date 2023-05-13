mod account;
mod guards;
mod status;
mod wasm;

use b3_helper::types::{SignerCanisterInitArgs, UserId};
use b3_signer_lib::{
    state::State,
    store::{with_owner, with_owner_mut, with_state, with_state_mut, with_wasm_mut},
};
use ic_cdk::{api::call::arg_data, export::candid::candid_method, init, post_upgrade, pre_upgrade};

#[init]
#[candid_method(init)]
pub fn init() {
    let (call_arg,) = arg_data::<(Option<SignerCanisterInitArgs>,)>();

    let owner = match call_arg {
        Some(args) => args.owner,
        None => ic_cdk::caller(),
    };

    with_state_mut(|s| s.init());

    with_owner_mut(|o| *o = owner);
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let owner = with_owner(|o| o.clone());
    let state = with_state(|s| s.clone());

    ic_cdk::storage::stable_save((state, owner)).unwrap();

    with_wasm_mut(|w| w.unload());
}

#[post_upgrade]
pub fn post_upgrade() {
    let (state_prev, owner_prev): (State, UserId) = ic_cdk::storage::stable_restore().unwrap();

    with_state_mut(|s| *s = state_prev);

    with_owner_mut(|o| *o = owner_prev);
}

#[cfg(test)]
mod tests {
    use b3_helper::types::*;
    use b3_signer_lib::{
        account::SignerAccount,
        ledger::network::Network,
        ledger::types::*,
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

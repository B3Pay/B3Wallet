mod account;
mod guards;
mod status;
mod wasm;

use b3_helper::types::SignerCanisterInitArgs;
use b3_signer_lib::{
    signer::{Roles, SignerUser},
    state::State,
    store::{with_signers, with_signers_mut, with_state, with_state_mut, with_wasm_mut},
    types::SignerUsers,
};
use ic_cdk::{api::call::arg_data, export::candid::candid_method, init, post_upgrade, pre_upgrade};

#[init]
#[candid_method(init)]
pub fn init() {
    let (call_arg,) = arg_data::<(Option<SignerCanisterInitArgs>,)>();

    let owner = SignerUser::from(Roles::Admin);

    match call_arg {
        Some(args) => {
            with_signers_mut(|signers| {
                signers.insert(args.owner_id, owner);

                if let Some(system_id) = args.system_id {
                    let name = "system".to_owned();
                    let system = SignerUser::new(Roles::Canister, Some(name));

                    signers.insert(system_id, system);
                }
            });
        }
        None => {
            let owner_id = ic_cdk::caller();

            with_signers_mut(|signers| {
                signers.insert(owner_id, owner);
            });
        }
    };

    with_state_mut(|state| state.init());
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let signers = with_signers(|o| o.clone());
    let state = with_state(|s| s.clone());

    ic_cdk::storage::stable_save((state, signers)).unwrap();

    with_wasm_mut(|wasm| wasm.unload());
}

#[post_upgrade]
pub fn post_upgrade() {
    let (state_prev, sign_prev): (State, SignerUsers) = ic_cdk::storage::stable_restore().unwrap();

    with_state_mut(|state| *state = state_prev);

    with_signers_mut(|signers| *signers = sign_prev);
}

#[cfg(test)]
mod tests {
    use b3_helper::types::*;
    use b3_signer_lib::{
        account::SignerAccount, ledger::network::Network, ledger::types::*,
        request::sign::SignRequest, signed::SignedTransaction, state::State, types::*,
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

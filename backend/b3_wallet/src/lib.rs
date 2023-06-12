mod account;
mod permit;
mod processed;
mod request;
mod status;
mod wasm;

use b3_helper_lib::{types::WalletCanisterInitArgs, wasm::with_wasm_mut};
use b3_permit_lib::{
    signer::{Roles, Signer},
    state::PrmitState,
    store::{with_permit, with_permit_mut},
};
use b3_wallet_lib::{
    state::WalletState,
    store::{with_wallet, with_wallet_mut},
};
use ic_cdk::{api::call::arg_data, export::candid::candid_method, init, post_upgrade, pre_upgrade};

#[init]
#[candid_method(init)]
pub fn init() {
    let (call_arg,) = arg_data::<(Option<WalletCanisterInitArgs>,)>();

    let owner = Signer::from(Roles::Admin);

    match call_arg {
        Some(args) => {
            with_permit_mut(|permit| {
                permit.signers.insert(args.owner_id, owner);

                if let Some(system_id) = args.system_id {
                    let name = "system".to_owned();
                    let system = Signer::new(Roles::Canister, Some(name), None);

                    permit.signers.insert(system_id, system);
                }
            });
        }
        None => {
            let owner_id = ic_cdk::caller();

            with_permit_mut(|permit| {
                permit.signers.insert(owner_id, owner);
            });
        }
    };

    with_wallet_mut(|state| state.init_wallet());
}

#[pre_upgrade]
pub fn pre_upgrade() {
    with_wasm_mut(|wasm| wasm.unload());

    let permit = with_permit(|o| o.clone());
    let state = with_wallet(|s| s.clone());

    ic_cdk::storage::stable_save((state, permit)).unwrap();
}

#[post_upgrade]
pub fn post_upgrade() {
    let (state_prev, sign_prev): (WalletState, PrmitState) =
        ic_cdk::storage::stable_restore().unwrap();

    with_wallet_mut(|state| *state = state_prev);

    with_permit_mut(|permit| *permit = sign_prev);
}

#[cfg(test)]
mod tests {
    use b3_helper_lib::environment::Environment;
    use b3_helper_lib::time::NanoTimeStamp;
    use b3_helper_lib::tokens::Tokens;
    use b3_helper_lib::types::*;
    use b3_permit_lib::processed::ProcessedRequest;
    use b3_permit_lib::request::{
        btc::transfer::*, icp::transfer::*, inner::account::*, inner::setting::*, inner::signer::*,
        request::Request,
    };
    use b3_permit_lib::signer::Roles;
    use b3_permit_lib::types::*;
    use b3_wallet_lib::account::WalletAccount;
    use b3_wallet_lib::ledger::btc::network::BtcNetwork;
    use b3_wallet_lib::ledger::ckbtc::types::*;
    use b3_wallet_lib::ledger::types::*;
    use b3_wallet_lib::types::*;

    use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, Satoshi, UtxoFilter};
    use ic_cdk::export::candid::export_service;

    #[test]
    fn generate_candid() {
        use std::io::Write;

        let mut file = std::fs::File::create("./b3_wallet.did").unwrap();

        export_service!();

        let candid = __export_service();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}

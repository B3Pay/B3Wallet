mod account;
mod permit;
mod processed;
mod request;
mod setting;
mod wallet;
mod wasm;

use b3_operations::{
    state::OperationState,
    store::{with_operation, with_operation_mut},
    types::UserMap,
    user::{role::UserRole, User},
};
use b3_utils::{
    types::{WalletCanisterInitArgs, WalletController},
    wasm::with_wasm_mut,
};
use b3_wallet_lib::{
    state::WalletState,
    store::{with_setting_mut, with_wallet, with_wallet_mut},
};
use candid::candid_method;
use ic_cdk::{api::call::arg_data, init, post_upgrade, pre_upgrade};

#[init]
#[candid_method(init)]
pub fn init() {
    // when the canister is created by another canister (e.g. the system canister)
    // this function is called with the arguments passed to the canister constructor.
    let (call_arg,) = arg_data::<(Option<WalletCanisterInitArgs>,)>();

    let mut signers = UserMap::new();

    let owner_id = match call_arg {
        Some(WalletCanisterInitArgs {
            owner_id,
            system_id,
        }) => {
            // if the canister is created by the system canister, the system canister
            // is added as trusted Canister
            signers.insert(
                system_id,
                User::new(UserRole::Canister, "System".to_owned(), None),
            );
            owner_id
        }
        None => ic_cdk::caller(),
    };

    signers.insert(
        owner_id,
        User::new(UserRole::Admin, "Owner".to_owned(), None),
    );

    with_operation_mut(|p| p.users = signers);
    // set initial controllers
    with_setting_mut(|s| {
        s.controllers
            .insert(ic_cdk::id(), WalletController::new("Self".to_owned(), None));

        s.controllers
            .insert(owner_id, WalletController::new("Owner".to_owned(), None));
    });
}

#[pre_upgrade]
pub fn pre_upgrade() {
    with_wasm_mut(|wasm| wasm.unload());

    let permit = with_operation(|o| o.clone());
    let state = with_wallet(|s| s.clone());

    ic_cdk::storage::stable_save((state, permit)).unwrap();
}

#[post_upgrade]
pub fn post_upgrade() {
    let (state_prev, sign_prev): (WalletState, OperationState) =
        ic_cdk::storage::stable_restore().unwrap();

    with_wallet_mut(|state| *state = state_prev);

    with_operation_mut(|permit| *permit = sign_prev);
}

#[cfg(test)]
mod tests {
    use b3_operations::operation::{
        btc::transfer::*, global::*, icp::transfer::*, inner::account::*, inner::setting::*,
        inner::user::*, Operation,
    };
    use b3_operations::processed::ProcessedOperation;
    use b3_operations::response::Response;
    use b3_operations::types::*;
    use b3_operations::user::role::UserRole;
    use b3_utils::currency::ICPToken;
    use b3_utils::currency::TokenAmount;
    use b3_utils::timestamp::NanoTimeStamp;
    use b3_utils::types::*;
    use b3_utils::Environment;

    use b3_utils::wasm::*;
    use b3_wallet_lib::account::WalletAccount;
    use b3_wallet_lib::ledger::btc::network::BtcNetwork;
    use b3_wallet_lib::ledger::ckbtc::types::*;
    use b3_wallet_lib::ledger::types::*;
    use b3_wallet_lib::types::*;

    use candid::export_service;
    use ic_cdk::api::management_canister::bitcoin::Satoshi;

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

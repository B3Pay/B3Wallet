mod account;
mod guards;
mod status;
mod wasm;

use b3_user_lib::{state::State, types::UserControlArgs, with_state, with_state_mut};

use guards::OWNER;
use wasm::WASM;

use ic_cdk::{
    api::call::arg_data,
    export::{candid::candid_method, Principal},
    init, post_upgrade, pre_upgrade,
};

#[init]
#[candid_method(init)]
pub fn init() {
    let call_arg = arg_data::<(Option<UserControlArgs>,)>().0;

    let owner = match call_arg {
        Some(args) => args.owner,
        None => ic_cdk::caller(),
    };

    OWNER.with(|s| {
        *s.borrow_mut() = owner;
    });
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let owner = OWNER.with(|s| s.borrow().clone());
    with_state(|s| {
        ic_cdk::storage::stable_save((s, owner)).unwrap();
    });

    WASM.with(|s| s.borrow_mut().reset());
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev, owner_prev): (State, Principal) = ic_cdk::storage::stable_restore().unwrap();
    with_state_mut(|s| {
        *s = s_prev;
    });

    OWNER.with(|s| {
        *s.borrow_mut() = owner_prev;
    });
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use crate::account::query::export_candid;
    use std::io::Write;

    let mut file = std::fs::File::create("./b3_user.did").unwrap();

    let candid = export_candid();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}

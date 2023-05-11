use std::cell::RefCell;

use b3_signer_lib::error::SignerError;
use candid::candid_method;
use ic_cdk::{
    api::management_canister::{
        main::{update_settings, UpdateSettingsArgument},
        provisional::CanisterSettings,
    },
    export::Principal,
    query, update,
};

use b3_shared::{
    b3_trap,
    error::TrapError,
    types::{ControllerId, UserId},
};

thread_local! {
    pub static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

pub fn caller_is_owner() -> Result<(), String> {
    let caller = ic_cdk::caller();
    let controllers: Principal = OWNER.with(|state| state.borrow().clone());

    if caller == controllers {
        Ok(())
    } else {
        Err(SignerError::CallerIsNotOwner.to_string())
    }
}

#[query]
#[candid_method(query)]
pub fn get_owner() -> UserId {
    OWNER.with(|s| s.borrow().clone())
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub fn change_owner(new_owner: UserId) -> UserId {
    OWNER.with(|s| {
        *s.borrow_mut() = new_owner;
    });

    OWNER.with(|s| s.borrow().clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn update_canister_controllers(mut controllers: Vec<ControllerId>) -> () {
    let canister_id = ic_cdk::id();
    let owner = OWNER.with(|s| *s.borrow());

    if !controllers.contains(&owner) {
        controllers.push(owner);
    }

    if !controllers.contains(&canister_id) {
        controllers.push(canister_id);
    }

    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };

    update_settings(arg)
        .await
        .map_err(|err| SignerError::UpdateSettingsError(err.1))
        .unwrap_or_else(|err| b3_trap(err))
}

use b3_helper::{
    b3_revert,
    error::TrapError,
    types::{ControllerId, UserId},
};
use b3_signer_lib::{
    error::SignerError,
    store::{with_owner, with_owner_mut},
};
use candid::candid_method;
use ic_cdk::{
    api::management_canister::{
        main::{update_settings, UpdateSettingsArgument},
        provisional::CanisterSettings,
    },
    query, update,
};

pub fn caller_is_owner() -> Result<(), String> {
    let caller = ic_cdk::caller();
    let owner = with_owner(|o| o.clone());

    if caller == owner {
        Ok(())
    } else {
        Err(SignerError::CallerIsNotOwner.to_string())
    }
}

#[query]
#[candid_method(query)]
pub fn get_owner() -> UserId {
    with_owner(|o| o.clone())
}

#[update(guard = "caller_is_owner")]
#[candid_method(update)]
pub fn change_owner(new_owner: UserId) -> UserId {
    with_owner_mut(|o| {
        *o = new_owner;
    });

    with_owner(|o| o.clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn update_canister_controllers(mut controllers: Vec<ControllerId>) -> () {
    let canister_id = ic_cdk::id();
    let owner = with_owner(|s| *s);

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
        .unwrap_or_else(|err| b3_revert(err))
}

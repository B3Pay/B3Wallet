use ic_cdk::export::candid::Principal;

use crate::{
    canister::{create_canister_install_code, update_user_control_controllers},
    store::with_state_mut,
    types::{UserControl, UserId},
};

pub const CREATE_USER_CANISTER_CYCLES: u128 = 1_000_000_000_000;

pub async fn new_user_control(user: UserId, system: Principal) -> Result<UserControl, String> {
    let wasm_arg = with_state_mut(|state| {
        state.init_user_control(user)?;

        state.get_wasm_arg(user)
    })?;

    let user_control_id = create_canister_install_code(
        Vec::from([system, user]),
        &wasm_arg,
        CREATE_USER_CANISTER_CYCLES,
    )
    .await;

    match user_control_id {
        Err(e) => {
            with_state_mut(|s| s.remove_user_control(&user));
            Err(format!("Error creating user control: {}!", e))
        }
        Ok(user_control_id) => {
            let user_control = with_state_mut(|s| s.add_user_control(user, user_control_id));

            update_user_control_controllers(user_control_id, user).await?;

            Ok(user_control)
        }
    }
}

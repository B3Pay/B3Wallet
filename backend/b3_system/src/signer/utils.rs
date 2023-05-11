use ic_cdk::api::management_canister::main::{
    create_canister_with_extra_cycles, install_code as ic_install_code, update_settings,
    CanisterId, CanisterInstallMode, CanisterSettings, CreateCanisterArgument, InstallCodeArgument,
    UpdateSettingsArgument,
};

use crate::store::with_state_mut;
use b3_shared::types::{ControllerId, InstallArg, Signer, SignerId, UserId};

pub const CREATE_USER_CANISTER_CYCLES: u128 = 1_000_000_000_000;

pub async fn new_signer(user: UserId) -> Result<Signer, String> {
    let system_id = ic_cdk::id();
    let wasm_arg = with_state_mut(|state| {
        state.init_signer(user)?;

        state.get_wasm_install_args(user)
    })?;

    let signer_id = create_canister_and_install_code(
        Vec::from([user, system_id]),
        &wasm_arg,
        CREATE_USER_CANISTER_CYCLES,
    )
    .await;

    match signer_id {
        Err(e) => {
            with_state_mut(|s| s.remove_signer(&user));
            Err(format!("Error creating user control: {}!", e))
        }
        Ok(signer_id) => {
            let signer = with_state_mut(|s| s.add_signer(user, signer_id));

            update_signer_controllers(signer_id, user).await?;

            Ok(signer)
        }
    }
}

pub async fn update_signer_controllers(signer_id: SignerId, user: UserId) -> Result<(), String> {
    let controllers = Vec::from([user, signer_id]);

    update_canister_controllers(signer_id, controllers.to_owned()).await
}

pub async fn create_canister_and_install_code(
    controllers: Vec<ControllerId>,
    wasm_arg: &InstallArg,
    cycles: u128,
) -> Result<SignerId, String> {
    let settings = Some(CanisterSettings {
        controllers: Some(controllers.clone()),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    });

    let result =
        create_canister_with_extra_cycles(CreateCanisterArgument { settings }, cycles).await;

    match result {
        Err((_, message)) => Err(format!("Failed to create canister: {}!", message)),
        Ok(result) => {
            let signer_id = result.0.canister_id;

            let install = install_code(signer_id, wasm_arg, CanisterInstallMode::Install).await;

            match install {
                Err(_) => Err(format!(
                    "Failed to install code on canister: {}!",
                    signer_id
                )),
                Ok(_) => Ok(signer_id),
            }
        }
    }
}

async fn install_code(
    canister_id: SignerId,
    InstallArg {
        wasm,
        arg: install_arg,
    }: &InstallArg,
    mode: CanisterInstallMode,
) -> Result<(), String> {
    let arg = InstallCodeArgument {
        mode,
        canister_id,
        wasm_module: wasm.clone(),
        arg: install_arg.clone(),
    };

    ic_install_code(arg).await.map_err(|(_, message)| message)
}

pub async fn update_canister_controllers(
    canister_id: CanisterId,
    controllers: Vec<ControllerId>,
) -> Result<(), String> {
    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };

    update_settings(arg).await.map_err(|(_, message)| message)
}

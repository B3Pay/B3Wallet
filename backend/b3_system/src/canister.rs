use crate::types::{CanisterStatus, UserControlId, UserId, WasmArg};
use candid::Principal;

use ic_cdk::api::management_canister::main::{
    canister_status as ic_canister_status, create_canister_with_extra_cycles,
    install_code as ic_install_code, update_settings, CanisterId, CanisterIdRecord,
    CanisterInstallMode, CanisterSettings, CreateCanisterArgument, InstallCodeArgument,
    UpdateSettingsArgument,
};
use ic_cdk::{api::call::CallResult, api::time, call};

pub async fn update_user_control_controllers(
    user_control_id: UserControlId,
    user: UserId,
) -> Result<(), String> {
    let controllers = Vec::from([user, user_control_id]);
    let result = update_canister_controllers(user_control_id, controllers.to_owned()).await;

    match result {
        Err((_, message)) => Err(format!(
            "Failed to update controllers for user control: {}!",
            message
        )),
        Ok(_) => Ok(()),
    }
}

pub async fn create_canister_install_code(
    controllers: Vec<Principal>,
    wasm_arg: &WasmArg,
    cycles: u128,
) -> Result<Principal, String> {
    let result = create_canister_with_extra_cycles(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(controllers.clone()),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            }),
        },
        cycles,
    )
    .await;

    match result {
        Err((_, message)) => Err(format!("Failed to create canister: {}!", message)),
        Ok(result) => {
            let canister_id = result.0.canister_id;

            let install = install_code(canister_id, wasm_arg, CanisterInstallMode::Install).await;

            match install {
                Err(_) => Err(format!(
                    "Failed to install code on canister: {}!",
                    canister_id
                )),
                Ok(_) => Ok(canister_id),
            }
        }
    }
}

async fn install_code(
    canister_id: Principal,
    WasmArg { wasm, install_arg }: &WasmArg,
    mode: CanisterInstallMode,
) -> CallResult<()> {
    let arg = InstallCodeArgument {
        mode,
        canister_id,
        wasm_module: wasm.get(),
        arg: install_arg.clone(),
    };

    ic_install_code(arg).await
}

pub async fn update_canister_controllers(
    canister_id: Principal,
    controllers: Vec<Principal>,
) -> CallResult<()> {
    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };

    update_settings(arg).await
}

pub async fn canister_wasm_version(canister_id: Principal) -> CallResult<String> {
    let res: (String,) = call(canister_id, "version", ()).await?;

    Ok(res.0)
}

pub async fn canister_status(canister_id: CanisterId) -> Result<CanisterStatus, String> {
    let version = canister_wasm_version(canister_id).await.unwrap_or_default();
    let status = ic_canister_status(CanisterIdRecord { canister_id }).await;

    match status {
        Ok((status,)) => Ok(CanisterStatus {
            id: canister_id,
            status,
            version,
            status_at: time(),
        }),
        Err((_, message)) => Err(format!("Failed to get status: {}!", message)),
    }
}

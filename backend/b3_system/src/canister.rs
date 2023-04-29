use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::main::{
    canister_status as ic_canister_status, create_canister_with_extra_cycles,
    install_code as ic_install_code, update_settings, CanisterId, CanisterIdRecord,
    CanisterInstallMode, CanisterSettings, CanisterStatusResponse, CreateCanisterArgument,
    InstallCodeArgument, UpdateSettingsArgument,
};
use ic_cdk::api::time;

use crate::control::{UserControlId, UserId};

pub struct WasmArg {
    pub wasm: Vec<u8>,
    pub install_arg: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct SegmentStatus {
    pub id: Principal,
    pub status: CanisterStatusResponse,
    pub status_at: u64,
}

pub type SegmentStatusResult = Result<SegmentStatus, String>;

/// Once mission control is created:
/// 1. we remove the console from the controllers because the data are owned by the developers
/// 2. we add the newly created mission control canister as its own controllers. that way it can add future controllers such as those the developers will add to interact with the terminal.
pub async fn update_user_control_controllers(
    user_control_id: &UserControlId,
    user: &UserId,
) -> Result<(), String> {
    let controllers = Vec::from([*user, *user_control_id]);
    let result = update_canister_controllers(*user_control_id, controllers.to_owned()).await;

    match result {
        Err(_) => Err("Failed to update the controllers of the mission control.".to_string()),
        Ok(_) => Ok(()),
    }
}

pub async fn create_canister_install_code(
    controllers: Vec<Principal>,
    wasm_arg: &WasmArg,
    cycles: u128,
) -> Result<Principal, String> {
    let record = create_canister_with_extra_cycles(
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

    match record {
        Err((_, message)) => Err(["Failed to create canister.", &message].join(" - ")),
        Ok(record) => {
            let canister_id = record.0.canister_id;

            let install = install_code(canister_id, wasm_arg, CanisterInstallMode::Install).await;

            match install {
                Err(_) => Err("Failed to install code in canister.".to_string()),
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
        wasm_module: wasm.clone(),
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

pub async fn segment_status(canister_id: CanisterId) -> SegmentStatusResult {
    let status = ic_canister_status(CanisterIdRecord { canister_id }).await;

    match status {
        Ok((status,)) => Ok(SegmentStatus {
            id: canister_id,
            status,
            status_at: time(),
        }),
        Err((_, message)) => Err(["Failed to get canister status: ".to_string(), message].join("")),
    }
}

use async_trait::async_trait;
use b3_helper_lib::{
    types::{CanisterId, WasmHashString, WasmVersion},
    wasm::with_wasm,
};
use b3_wallet_lib::error::WalletError;
use ic_cdk::{
    api::management_canister::{
        main::{
            install_code, update_settings, CanisterInstallMode, InstallCodeArgument,
            UpdateSettingsArgument,
        },
        provisional::CanisterSettings,
    },
    export::{candid::CandidType, serde::Deserialize},
};

use crate::{
    error::PermitError,
    request::{request::RequestTrait, result::ExecutionResult},
};

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

// UPDATE SETTINGS - START
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UpdateCanisterSettings {
    pub canister_id: CanisterId,
    pub settings: CanisterSettings,
}

impl From<&UpdateCanisterSettings> for UpdateSettingsArgument {
    fn from(args: &UpdateCanisterSettings) -> Self {
        UpdateSettingsArgument {
            canister_id: args.canister_id,
            settings: args.settings.clone(),
        }
    }
}

#[async_trait]
impl RequestTrait for UpdateCanisterSettings {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let args = UpdateSettingsArgument::from(&self);

        update_settings(args)
            .await
            .map_err(|err| WalletError::UpdateSettingsError(err.1))?;

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        let canister_id = ic_cdk_id();

        // first check the controller is passed and then check if the canister is in the list of controllers
        if let Some(controller) = self.settings.controllers.as_ref() {
            if !controller.contains(&canister_id) {
                return Err(PermitError::InvalidController);
            }
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "update_canister_settings".to_string()
    }

    fn title(&self) -> String {
        format!("Update canister settings for {}", self.canister_id)
    }
}

// UPDATE SETTINGS - END

// UPGRADE CANISTER - START
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UpgradeCanister {
    pub wasm_version: WasmVersion,
    pub wasm_hash_string: WasmHashString,
}

impl UpgradeCanister {
    pub fn new(wasm_hash_string: WasmHashString, wasm_version: WasmVersion) -> Self {
        UpgradeCanister {
            wasm_hash_string,
            wasm_version,
        }
    }
}

#[async_trait]
impl RequestTrait for UpgradeCanister {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let canister_id = ic_cdk_id();
        let wasm_module = with_wasm(|w| w.get());

        let args = InstallCodeArgument {
            canister_id,
            wasm_module,
            arg: Vec::new(),
            mode: CanisterInstallMode::Upgrade,
        };

        install_code(args).await.unwrap();

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        with_wasm(|w| {
            if w.is_empty() {
                return Err(PermitError::WasmNotSet);
            }

            if w.generate_hash_string() != self.wasm_hash_string {
                return Err(PermitError::InvalidWasmHash);
            }

            Ok(())
        })
    }

    fn method_name(&self) -> String {
        "upgrade_canister".to_string()
    }

    fn title(&self) -> String {
        format!("Upgrade canister to version {}", self.wasm_version)
    }
}

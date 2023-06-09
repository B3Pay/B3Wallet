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
    error::RequestError,
    pending::RequestTrait,
    types::{ConsentInfo, ConsentMessageResponse},
};

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

// UPDATE SETTINGS - START
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UpdateCanisterSettingsRequest {
    pub canister_id: CanisterId,
    pub settings: CanisterSettings,
}

impl From<&UpdateCanisterSettingsRequest> for UpdateSettingsArgument {
    fn from(args: &UpdateCanisterSettingsRequest) -> Self {
        UpdateSettingsArgument {
            canister_id: args.canister_id,
            settings: args.settings.clone(),
        }
    }
}

#[async_trait]
impl RequestTrait for UpdateCanisterSettingsRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        update_settings(self.into())
            .await
            .map_err(|err| WalletError::UpdateSettingsError(err.1))?;

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!("Canister {} settings updated", self.canister_id),
        }))
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        let canister_id = ic_cdk_id();

        // first check the controller is passed and then check if the canister is in the list of controllers
        if let Some(controller) = self.settings.controllers.as_ref() {
            if !controller.contains(&canister_id) {
                return Err(RequestError::InvalidController);
            }
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "update_canister_settings".to_string()
    }
}

// UPDATE SETTINGS - END

// UPGRADE CANISTER - START
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UpgradeCanisterRequest {
    pub wasm_version: WasmVersion,
    pub wasm_hash_string: WasmHashString,
}

impl UpgradeCanisterRequest {
    pub fn new(wasm_hash_string: WasmHashString, wasm_version: WasmVersion) -> Self {
        UpgradeCanisterRequest {
            wasm_hash_string,
            wasm_version,
        }
    }
}

#[async_trait]
impl RequestTrait for UpgradeCanisterRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let canister_id = ic_cdk_id();
        let wasm_module = with_wasm(|w| w.get());

        let args = InstallCodeArgument {
            canister_id,
            wasm_module,
            arg: Vec::new(),
            mode: CanisterInstallMode::Upgrade,
        };

        install_code(args).await.unwrap();

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!(
                "Canister {} upgraded to version {}, hash {}",
                canister_id, self.wasm_version, self.wasm_hash_string
            ),
        }))
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        with_wasm(|w| {
            if w.is_empty() {
                return Err(RequestError::WasmNotSet);
            }

            if w.generate_hash_string() != self.wasm_hash_string {
                return Err(RequestError::InvalidWasmHash);
            }

            Ok(())
        })
    }

    fn method_name(&self) -> String {
        "upgrade_canister".to_string()
    }
}

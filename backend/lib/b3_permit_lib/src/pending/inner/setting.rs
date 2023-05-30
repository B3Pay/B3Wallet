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
    pending::Request,
    types::{ConsendInfo, ConsentMessageResponse},
};

use super::InnerRequest;

// UPDATE SETTINGS - START
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
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

impl From<UpdateCanisterSettingsRequest> for Request {
    fn from(args: UpdateCanisterSettingsRequest) -> Self {
        InnerRequest::UpdateCanisterSettingsRequest(args).into()
    }
}

impl UpdateCanisterSettingsRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        update_settings(self.into())
            .await
            .map_err(|err| WalletError::UpdateSettingsError(err.1))?;

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Canister {} settings updated", self.canister_id),
            ..Default::default()
        }))
    }

    pub fn validate_request(&self) -> Result<(), WalletError> {
        let canister_id = ic_cdk::id();

        // first check the controller is passed and then check if the controller is in the list of controllers
        if let Some(controller) = self.settings.controllers.as_ref() {
            if !controller.contains(&canister_id) {
                return Err(WalletError::InvalidController);
            }
        }

        Ok(())
    }
}

// UPDATE SETTINGS - END

// UPGRADE CANISTER - START
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
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

impl From<UpgradeCanisterRequest> for Request {
    fn from(args: UpgradeCanisterRequest) -> Self {
        InnerRequest::UpgradeCanisterRequest(args).into()
    }
}

impl UpgradeCanisterRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let canister_id = ic_cdk::id();
        let wasm_module = with_wasm(|w| w.get());

        let args = InstallCodeArgument {
            canister_id,
            wasm_module,
            arg: Vec::new(),
            mode: CanisterInstallMode::Upgrade,
        };

        install_code(args).await.unwrap();

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!(
                "Canister {} upgraded to version {}, hash {}",
                canister_id, self.wasm_version, self.wasm_hash_string
            ),
            ..Default::default()
        }))
    }
}

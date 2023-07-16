use crate::{error::SystemError, types::Controllers};
use b3_helper_lib::{
    ic_canister_status,
    types::{
        CanisterId, SignerId, WalletCanisterInstallArg, WalletCanisterStatus, WalletVersion,
        WasmHash,
    },
};
use ic_cdk::{
    api::management_canister::{
        main::{install_code, update_settings, InstallCodeArgument, UpdateSettingsArgument},
        provisional::CanisterSettings,
    },
    export::candid::{CandidType, Deserialize},
};

#[derive(CandidType, Deserialize, PartialEq, Clone)]
pub struct WalletCanister(pub CanisterId);

impl From<CanisterId> for WalletCanister {
    fn from(canister_id: CanisterId) -> Self {
        Self(canister_id)
    }
}

impl WalletCanister {
    /// Get the owner of the canister.
    pub async fn validate_signer(&self, signer_id: SignerId) -> Result<bool, SystemError> {
        let (validate,): (bool,) = ic_cdk::call(self.0, "validate_signer", (signer_id,))
            .await
            .map_err(|err| SystemError::ValidateSignerError(err.1))?;

        Ok(validate)
    }

    /// Get the wasm hash of the canister.
    pub async fn wasm_hash(&self) -> Result<WasmHash, SystemError> {
        let (wasm_hash,): (WasmHash,) = ic_cdk::call(self.0, "wasm_hash", ())
            .await
            .map_err(|err| SystemError::WasmHashError(err.1))?;

        Ok(wasm_hash)
    }

    /// Get the version of the canister.
    pub async fn version(&self) -> Result<WalletVersion, SystemError> {
        let (version,): (WalletVersion,) = ic_cdk::call(self.0, "version", ())
            .await
            .map_err(|err| SystemError::VersionError(err.1))?;

        Ok(version)
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn status(&self) -> Result<WalletCanisterStatus, SystemError> {
        let (canister_status,): (WalletCanisterStatus,) = ic_cdk::call(self.0, "status", ())
            .await
            .map_err(|err| SystemError::CanisterStatusError(err.1))?;

        Ok(canister_status)
    }

    /// Install the code for the canister.
    pub async fn install_code(&self, args: WalletCanisterInstallArg) -> Result<(), SystemError> {
        let canister_id = self.0;

        let install_args = InstallCodeArgument {
            arg: args.arg,
            mode: args.mode,
            wasm_module: args.wasm_module,
            canister_id,
        };

        install_code(install_args)
            .await
            .map_err(|err| SystemError::InstallCodeError(err.1))
    }

    /// Update the controllers of the canister.
    /// The caller must be a controller of the canister.
    /// Default controllers are the owner and the signer itself.
    pub async fn add_controllers(&self, mut controllers: Controllers) -> Result<(), SystemError> {
        let canister_id = self.0;

        let canister_status = ic_canister_status(canister_id)
            .await
            .map_err(|err| SystemError::CanisterStatusError(err.to_string()))?;

        if !controllers.contains(&canister_id) {
            controllers.push(canister_id);
            canister_status
                .settings
                .controllers
                .iter()
                .for_each(|controller| {
                    if !controllers.contains(controller) {
                        controllers.push(controller.clone());
                    }
                });
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
            .map_err(|err| SystemError::UpdateCanisterControllersError(err.1))
    }
}

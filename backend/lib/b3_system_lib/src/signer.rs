use crate::types::SignerCanister;
use b3_helper_lib::{
    constants::RATE_LIMIT,
    error::HelperError,
    types::{
        CanisterId, ControllerId, SignerCanisterInstallArg, SignerCanisterStatus, SignerId,
        Version, WasmHash,
    },
};
use ic_cdk::api::{
    management_canister::{
        main::{
            create_canister_with_extra_cycles, install_code, update_settings,
            CreateCanisterArgument, InstallCodeArgument, UpdateSettingsArgument,
        },
        provisional::CanisterSettings,
    },
    time,
};

impl From<CanisterId> for SignerCanister {
    fn from(canister_id: CanisterId) -> Self {
        Self {
            canister_id: Some(canister_id),
            updated_at: time(),
            created_at: time(),
        }
    }
}

impl SignerCanister {
    /// Create a new canister.
    pub fn new() -> Self {
        let now = time();

        Self {
            canister_id: None,
            updated_at: now,
            created_at: now,
        }
    }

    /// get with updated_at.
    pub fn get_with_update_rate(&mut self) -> Result<SignerCanister, HelperError> {
        self.check_rate()?;
        self.updated_at = time();

        Ok(self.clone())
    }

    /// Set the canister id.
    pub fn set_canister_id(&mut self, canister_id: CanisterId) {
        self.canister_id = Some(canister_id);
        self.updated_at = time();
    }

    /// Reset the canister id.
    /// This is used when the installation of the canister fails.
    pub fn reset_canister_id(&mut self) {
        self.canister_id = None;
        self.updated_at = time();
    }

    /// Returns the canister id, throws an error if it is not available.
    pub fn canister_id(&self) -> Result<CanisterId, HelperError> {
        match self.canister_id {
            Some(canister_id) => Ok(canister_id),
            None => Err(HelperError::SignerNotAvailable),
        }
    }

    /// Make an function that use updated_at and check the rate of the user.
    pub fn check_rate(&self) -> Result<(), HelperError> {
        let now = time();
        let updated_at = self.updated_at;

        if now - updated_at < RATE_LIMIT {
            Err(HelperError::RateLimitExceeded)
        } else {
            Ok(())
        }
    }

    /// Get the owner of the canister.
    pub async fn get_owner(&self) -> Result<SignerId, HelperError> {
        let canister_id = self.canister_id()?;

        let (owner,): (SignerId,) = ic_cdk::call(canister_id, "get_owner", ())
            .await
            .map_err(|(_, message)| HelperError::GetOwnerError(message))?;

        Ok(owner)
    }

    /// Get the wasm hash of the canister.
    pub async fn wasm_hash(&self) -> Result<WasmHash, HelperError> {
        let canister_id = self.canister_id()?;

        let (wasm_hash,): (WasmHash,) = ic_cdk::call(canister_id, "wasm_hash", ())
            .await
            .map_err(|(_, message)| HelperError::WasmHashError(message))?;

        Ok(wasm_hash)
    }

    /// Get the version of the canister.
    pub async fn version(&self) -> Result<Version, HelperError> {
        let canister_id = self.canister_id()?;

        let (version,): (Version,) = ic_cdk::call(canister_id, "version", ())
            .await
            .map_err(|(_, message)| HelperError::VersionError(message))?;

        Ok(version)
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn status(&self) -> Result<SignerCanisterStatus, HelperError> {
        let canister_id = self.canister_id()?;

        let (canister_status,): (SignerCanisterStatus,) = ic_cdk::call(canister_id, "status", ())
            .await
            .map_err(|(_, message)| HelperError::CanisterStatusError(message))?;

        Ok(canister_status)
    }

    /// create a new canister and save the canister id.
    pub async fn create_with_cycles(
        &mut self,
        controllers: Vec<ControllerId>,
        cycles: u128,
    ) -> Result<CanisterId, HelperError> {
        let settings = Some(CanisterSettings {
            controllers: Some(controllers.clone()),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        });

        let result =
            create_canister_with_extra_cycles(CreateCanisterArgument { settings }, cycles).await;

        match result {
            Err((_, message)) => Err(HelperError::CreateCanisterError(message)),
            Ok(result) => {
                let canister_id = result.0.canister_id;

                self.canister_id = Some(canister_id);

                Ok(canister_id)
            }
        }
    }

    /// Install the code for the canister.
    pub async fn install_code(
        &mut self,
        SignerCanisterInstallArg {
            arg,
            mode,
            wasm_module,
        }: SignerCanisterInstallArg,
    ) -> Result<(), HelperError> {
        let canister_id = self.canister_id()?;

        let install_args = InstallCodeArgument {
            arg,
            mode,
            wasm_module,
            canister_id,
        };

        install_code(install_args)
            .await
            .map_err(|(_, message)| HelperError::InstallCodeError(message))
    }

    /// Update the controllers of the canister.
    /// The caller must be a controller of the canister.
    /// Default controllers are the owner and the signer itself.
    pub async fn update_controllers(
        &self,
        mut controllers: Vec<ControllerId>,
    ) -> Result<(), HelperError> {
        let canister_id = self.canister_id()?;

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
            .map_err(|(_, message)| HelperError::UpdateCanisterControllersError(message))
    }
}

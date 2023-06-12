use crate::{error::SystemError, types::WalletCanister};
use b3_helper_lib::{
    constants::RATE_LIMIT,
    time::NanoTimeStamp,
    types::{
        CanisterId, ControllerId, SignerId, Version, WalletCanisterInstallArg,
        WalletCanisterStatus, WasmHash,
    },
};
use ic_cdk::api::management_canister::{
    main::{
        create_canister_with_extra_cycles, install_code, update_settings, CreateCanisterArgument,
        InstallCodeArgument, UpdateSettingsArgument,
    },
    provisional::CanisterSettings,
};

impl From<CanisterId> for WalletCanister {
    fn from(canister_id: CanisterId) -> Self {
        Self {
            canisters: vec![canister_id],
            updated_at: NanoTimeStamp::now(),
            created_at: NanoTimeStamp::now(),
        }
    }
}

impl WalletCanister {
    /// Create a new canister.
    pub fn new() -> Self {
        Self {
            canisters: vec![],
            updated_at: NanoTimeStamp::now(),
            created_at: NanoTimeStamp::now(),
        }
    }

    /// get with updated_at.
    pub fn get_with_update_rate(&mut self) -> Result<WalletCanister, SystemError> {
        self.check_rate()?;
        self.updated_at = NanoTimeStamp::now();

        Ok(self.clone())
    }

    /// Set the canister id.
    pub fn add_canister_id(&mut self, canister_id: CanisterId) {
        self.canisters.push(canister_id);
        self.updated_at = NanoTimeStamp::now();
    }

    /// Returns the canister ids, throws an error if it is not available.
    pub fn canister_id(&self) -> Result<CanisterId, SystemError> {
        self.canisters
            .first()
            .cloned()
            .ok_or(SystemError::NoCanisterAvailable)
    }

    /// Make an function that use updated_at and check the rate of the user.
    pub fn check_rate(&self) -> Result<(), SystemError> {
        if self.updated_at.rate_limit_exceeded(RATE_LIMIT) {
            return Err(SystemError::RateLimitExceeded);
        } else {
            Ok(())
        }
    }

    /// Get the owner of the canister.
    pub async fn validate_signer(&self, signer_id: SignerId) -> Result<bool, SystemError> {
        let canister_id = self.canister_id()?;

        let (validate,): (bool,) = ic_cdk::call(canister_id, "validate_signer", (signer_id,))
            .await
            .map_err(|err| SystemError::ValidateSignerError(err.1))?;

        Ok(validate)
    }

    /// Get the wasm hash of the canister.
    pub async fn wasm_hash(&self) -> Result<WasmHash, SystemError> {
        let canister_id = self.canister_id()?;

        let (wasm_hash,): (WasmHash,) = ic_cdk::call(canister_id, "wasm_hash", ())
            .await
            .map_err(|err| SystemError::WasmHashError(err.1))?;

        Ok(wasm_hash)
    }

    /// Get the version of the canister.
    pub async fn version(&self) -> Result<Version, SystemError> {
        let canister_id = self.canister_id()?;

        let (version,): (Version,) = ic_cdk::call(canister_id, "version", ())
            .await
            .map_err(|err| SystemError::VersionError(err.1))?;

        Ok(version)
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn status(&self) -> Result<WalletCanisterStatus, SystemError> {
        let canister_id = self.canister_id()?;

        let (canister_status,): (WalletCanisterStatus,) = ic_cdk::call(canister_id, "status", ())
            .await
            .map_err(|err| SystemError::CanisterStatusError(err.1))?;

        Ok(canister_status)
    }

    /// create a new canister and save the canister id.
    pub async fn create_with_cycles(
        &mut self,
        controllers: Vec<ControllerId>,
        cycles: u128,
    ) -> Result<CanisterId, SystemError> {
        let settings = Some(CanisterSettings {
            controllers: Some(controllers.clone()),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        });

        let result =
            create_canister_with_extra_cycles(CreateCanisterArgument { settings }, cycles).await;

        match result {
            Ok(result) => {
                let canister_id = result.0.canister_id;

                self.add_canister_id(canister_id.clone());

                Ok(canister_id)
            }
            Err(err) => Err(SystemError::CreateCanisterError(err.1)),
        }
    }

    /// Install the code for the canister.
    pub async fn install_code(
        &mut self,
        WalletCanisterInstallArg {
            arg,
            mode,
            wasm_module,
        }: WalletCanisterInstallArg,
    ) -> Result<(), SystemError> {
        let canister_id = self.canister_id()?;

        let install_args = InstallCodeArgument {
            arg,
            mode,
            wasm_module,
            canister_id,
        };

        install_code(install_args)
            .await
            .map_err(|err| SystemError::InstallCodeError(err.1))
    }

    /// Update the controllers of the canister.
    /// The caller must be a controller of the canister.
    /// Default controllers are the owner and the signer itself.
    pub async fn update_controllers(
        &self,
        mut controllers: Vec<ControllerId>,
    ) -> Result<(), SystemError> {
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
            .map_err(|err| SystemError::UpdateCanisterControllersError(err.1))
    }
}

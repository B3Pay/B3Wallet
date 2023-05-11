use ic_cdk::api::{
    management_canister::{
        main::{
            create_canister_with_extra_cycles, install_code, update_settings, CanisterInstallMode,
            CreateCanisterArgument, InstallCodeArgument, UpdateSettingsArgument,
        },
        provisional::CanisterSettings,
    },
    time,
};

use crate::{
    error::SharedError,
    types::{Canister, CanisterId, CanisterStatus, ControllerId, InstallArg, UserId, Version},
};

impl From<UserId> for Canister {
    fn from(owner: UserId) -> Self {
        let now = time();

        Self {
            canister_id: None,
            created_at: now,
            updated_at: now,
            owner,
        }
    }
}

impl Canister {
    pub fn canister_id(&self) -> Result<CanisterId, SharedError> {
        match self.canister_id {
            Some(canister_id) => Ok(canister_id),
            None => Err(SharedError::SignerNotAvailable),
        }
    }

    pub fn set_canister_id(&mut self, canister_id: CanisterId) {
        self.canister_id = Some(canister_id);
        self.updated_at = time();
    }

    /// create a new canister.
    pub async fn create_with_cycles(
        &mut self,
        controllers: Vec<ControllerId>,
        cycles: u128,
    ) -> Result<CanisterId, SharedError> {
        let settings = Some(CanisterSettings {
            controllers: Some(controllers.clone()),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        });

        let result =
            create_canister_with_extra_cycles(CreateCanisterArgument { settings }, cycles).await;

        match result {
            Err((_, message)) => Err(SharedError::CreateCanisterError(message)),
            Ok(result) => {
                let canister_id = result.0.canister_id;

                self.set_canister_id(canister_id);

                Ok(canister_id)
            }
        }
    }

    /// Install the code for the canister.
    pub async fn install_code(
        &mut self,
        InstallArg { wasm, arg }: &InstallArg,
        mode: CanisterInstallMode,
    ) -> Result<(), SharedError> {
        let canister_id = self.canister_id()?;

        let install_args = InstallCodeArgument {
            mode,
            canister_id,
            wasm_module: wasm.clone(),
            arg: arg.to_vec(),
        };

        install_code(install_args)
            .await
            .map_err(|(_, message)| SharedError::InstallCodeError(message))
    }

    /// Get the version of the canister.
    pub async fn version(&self) -> Result<Version, SharedError> {
        let canister_id = self.canister_id()?;

        let (version,): (Version,) = ic_cdk::call(canister_id, "version", ())
            .await
            .map_err(|(_, message)| SharedError::VersionError(message))?;

        Ok(version)
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn status(&self) -> Result<CanisterStatus, SharedError> {
        let canister_id = self.canister_id()?;

        let (canister_status,): (CanisterStatus,) =
            ic_cdk::call(canister_id, "status", ())
                .await
                .map_err(|(_, message)| SharedError::CanisterStatusError(message))?;

        Ok(canister_status)
    }

    /// Update the controllers of the canister.
    /// The caller must be a controller of the canister.
    /// Default controllers are the owner and the signer itself.
    pub async fn update_controllers(
        &self,
        mut controllers: Vec<ControllerId>,
    ) -> Result<(), SharedError> {
        let canister_id = self.canister_id()?;

        if !controllers.contains(&self.owner) {
            controllers.push(self.owner);
        }

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
            .map_err(|(_, message)| SharedError::UpdateCanisterControllersError(message))
    }
}

use crate::{
    error::SystemError,
    release::Release,
    store::State,
    types::UserStates,
    types::{ReleaseVersion, Users},
    user::UserState,
};
use b3_utils::{
    ledger::types::{WalletCanisterInitArgs, WalletCanisterInstallArg},
    types::{CanisterId, CanisterIds, UserId},
};
use ic_cdk::api::management_canister::main::CanisterInstallMode;

impl State {
    // user
    pub fn init_user(&mut self, user: UserId) -> Result<UserState, SystemError> {
        if let Some(user_state) = self.users.get(&user) {
            if !user_state.canisters().is_empty() {
                return Err(SystemError::UserAlreadyExists);
            }
        }

        let user_state = UserState::new(None);

        self.users.insert(user, user_state.clone());

        Ok(user_state)
    }

    pub fn get_or_init_user(
        &mut self,
        user: UserId,
        opt_canister_id: Option<CanisterId>,
    ) -> Result<UserState, SystemError> {
        if let Some(mut states) = self.users.get(&user) {
            let mut user_state = states.update_rate()?;

            if let Some(canister_id) = opt_canister_id {
                user_state.add_canister(canister_id);
            }

            return Ok(user_state);
        }

        let user_state = UserState::new(opt_canister_id);

        self.users.insert(user, user_state.clone());

        Ok(user_state)
    }

    pub fn add_user(&mut self, user: UserId, user_state: UserState) {
        self.users.insert(user, user_state);
    }

    pub fn remove_user(&mut self, user: &UserId) {
        self.users.remove(user);
    }

    pub fn user_ids(&self) -> Users {
        self.users.iter().map(|(k, _)| k).collect()
    }

    pub fn wallet_canisters(&self) -> CanisterIds {
        self.users
            .iter()
            .map(|(_, v)| v.canisters())
            .flatten()
            .collect()
    }

    pub fn user_state(&self, user_id: UserId) -> Result<UserState, SystemError> {
        self.users
            .get(&user_id)
            .ok_or(SystemError::UserNotFound)
            .map(|state| state.clone())
    }

    pub fn user_states(&self) -> UserStates {
        self.users.iter().map(|(_, v)| v.clone()).collect()
    }

    pub fn number_of_users(&self) -> u64 {
        self.users.len()
    }

    // release
    pub fn get_release(&self, version: &ReleaseVersion) -> Result<Release, SystemError> {
        self.releases
            .get(version)
            .ok_or(SystemError::ReleaseNotFound)
    }

    pub fn get_release_install_args(
        &self,
        version: &ReleaseVersion,
        mode: CanisterInstallMode,
        init_args: WalletCanisterInitArgs,
    ) -> Result<WalletCanisterInstallArg, SystemError> {
        let wasm_module = self.get_release(version)?.wasm()?.bytes();

        let arg = init_args
            .encode()
            .map_err(|e| SystemError::InstallArgError(e.to_string()))?;

        Ok(WalletCanisterInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }

    pub fn latest_release(&self) -> Result<Release, SystemError> {
        self.releases
            .last_key_value()
            .ok_or(SystemError::ReleaseNotFound)
            .map(|(_, release)| release)
    }

    pub fn get_latest_install_args(
        &self,
        mode: CanisterInstallMode,
        init_args: WalletCanisterInitArgs,
    ) -> Result<WalletCanisterInstallArg, SystemError> {
        let wasm_module = self.latest_release()?.wasm()?.bytes();

        let arg = init_args
            .encode()
            .map_err(|e| SystemError::InstallArgError(e.to_string()))?;

        Ok(WalletCanisterInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }
}

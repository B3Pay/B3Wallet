use crate::{
    error::SystemError,
    types::{Canisters, Controllers, State, UserStates},
    types::{Release, Users},
    user::UserState,
    wallet::WalletCanister,
};
use b3_helper_lib::{
    release::ReleaseTypes,
    types::{
        CanisterId, ControllerId, SignerId, WalletCanisterInitArgs, WalletCanisterInstallArg,
        WalletVersion,
    },
};
use ic_cdk::api::management_canister::main::CanisterInstallMode;

impl State {
    // user
    pub fn init_user(&mut self, user: SignerId) -> Result<UserState, SystemError> {
        let canister = self.users.get(&user);

        if canister.is_some() {
            return Err(SystemError::UserAlreadyExists);
        }

        let user_state = UserState::new(None);

        self.users.insert(user, user_state.clone());

        Ok(user_state)
    }

    pub fn get_or_init_user(
        &mut self,
        user: SignerId,
        opt_canister_id: Option<CanisterId>,
    ) -> Result<UserState, SystemError> {
        if let Some(states) = self.users.get_mut(&user) {
            let mut user_state = states.update_rate()?;

            if let Some(canister_id) = opt_canister_id {
                user_state.add_canister(WalletCanister(canister_id));
            }

            return Ok(user_state);
        }

        let user_state = UserState::new(opt_canister_id);

        self.users.insert(user, user_state.clone());

        Ok(user_state)
    }

    pub fn add_user(&mut self, user: SignerId, user_state: UserState) {
        self.users.insert(user, user_state);
    }

    pub fn remove_user(&mut self, user: &SignerId) {
        self.users.remove(user);
    }

    pub fn user_ids(&self) -> Users {
        self.users.keys().cloned().collect()
    }

    pub fn wallet_canisters(&self) -> Canisters {
        self.users
            .values()
            .filter_map(|u| u.canisters().ok())
            .flatten()
            .collect()
    }

    pub fn user_state(&self, user_id: SignerId) -> Result<UserState, SystemError> {
        self.users
            .get(&user_id)
            .cloned()
            .ok_or(SystemError::UserNotFound)
    }

    pub fn user_states(&self) -> UserStates {
        self.users.values().cloned().collect()
    }

    pub fn number_of_users(&self) -> usize {
        self.users.len()
    }

    // controller
    pub fn get_controllers(&self) -> Controllers {
        self.controllers.clone()
    }

    pub fn add_controller(&mut self, controller_id: ControllerId) {
        self.controllers.push(controller_id);
    }

    pub fn remove_controller(&mut self, controller_id: ControllerId) {
        self.controllers.retain(|c| c != &controller_id);
    }

    // release
    pub fn get_release(&self, name: ReleaseTypes, version: &str) -> Result<&Release, SystemError> {
        let releases = self
            .releases
            .get(&name)
            .ok_or(SystemError::ReleaseNameNotFound)?;

        releases
            .iter()
            .find(|r| r.version == version)
            .ok_or(SystemError::ReleaseNotFound)
    }

    pub fn get_release_install_args(
        &self,
        name: ReleaseTypes,
        version: &WalletVersion,
        mode: CanisterInstallMode,
        init_args: WalletCanisterInitArgs,
    ) -> Result<WalletCanisterInstallArg, SystemError> {
        let wasm_module = self.get_release(name, version)?.wasm()?;

        let arg = init_args
            .encode()
            .map_err(|e| SystemError::InstallArgError(e.to_string()))?;

        Ok(WalletCanisterInstallArg {
            wasm_module,
            arg,
            mode,
        })
    }

    pub fn latest_release(&self, name: ReleaseTypes) -> Result<&Release, SystemError> {
        self.releases
            .get(&name)
            .ok_or(SystemError::ReleaseNameNotFound)?
            .last()
            .ok_or(SystemError::ReleaseNotFound)
    }

    pub fn get_latest_install_args(
        &self,
        name: ReleaseTypes,
        mode: CanisterInstallMode,
        init_args: WalletCanisterInitArgs,
    ) -> Result<WalletCanisterInstallArg, SystemError> {
        let wasm_module = self.latest_release(name)?.wasm()?;

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

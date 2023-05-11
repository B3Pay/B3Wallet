use crate::{error::SystemError, types::Release, types::State};
use b3_shared::{
    error::TrapError,
    types::{ControllerId, InstallArg, UserId, Wasm},
};

impl State {
    pub fn get_controllers(&self) -> Vec<ControllerId> {
        self.controllers.clone()
    }

    pub fn add_controller(&mut self, controller_id: ControllerId) {
        self.controllers.push(controller_id);
    }

    pub fn remove_controller(&mut self, controller_id: ControllerId) {
        self.controllers.retain(|c| c != &controller_id);
    }

    pub fn latest_release(&self) -> Result<&Release, SystemError> {
        self.releases.last().ok_or(SystemError::ReleaseNotFound)
    }

    pub fn get_latest_install_args(&self, owner: UserId) -> Result<InstallArg, SystemError> {
        let wasm: Wasm = self.latest_release()?.get_wasm()?;

        InstallArg::try_from((owner, wasm)).map_err(|e| SystemError::InstallArgError(e.to_string()))
    }
}

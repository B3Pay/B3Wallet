use crate::{
    types::Release,
    types::{State, UserControlArgs},
};
use b3_shared::types::{ControllerId, InstallArg, UserId, Wasm};
use ic_cdk::export::candid::Encode;

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

    pub fn latest_release(&self) -> Result<&Release, String> {
        self.releases.last().ok_or("No releases found!".to_string())
    }

    pub fn get_wasm_install_args(&self, user: UserId) -> Result<InstallArg, String> {
        let wasm: Wasm = self.latest_release()?.get_wasm()?;

        let install_arg: Vec<u8> = Encode!(&UserControlArgs {
            owner: user.to_owned()
        })
        .map_err(|e| format!("Failed to encode install arg: {}!", e))?;

        Ok(InstallArg {
            wasm,
            arg: install_arg,
        })
    }
}

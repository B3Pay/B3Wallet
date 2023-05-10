use crate::{
    types::{
        ControllerId, Controllers, UserControl, UserControlArgs, UserControlId, UserControlMap,
        UserId, WasmArg,
    },
    types::{Release, Releases, Wasm},
};
use ic_cdk::{
    api::time,
    export::candid::{CandidType, Deserialize, Encode},
};

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct State {
    pub releases: Releases,
    pub controllers: Controllers,
    pub user_controls: UserControlMap,
}

impl State {
    pub fn init_user_control(&mut self, user: UserId) -> Result<UserControl, String> {
        let user_control_id = self.get_user_control_id(&user);

        if let Some(id) = user_control_id {
            return Err(format!("User control already exists: {}!", id));
        }

        let now = time();

        let user_control = UserControl {
            owner: user,
            user_control_id: None,
            created_at: now,
            updated_at: now,
        };

        self.user_controls.insert(user, user_control.clone());

        Ok(user_control)
    }

    pub fn add_user_control(
        &mut self,
        user: UserId,
        user_control_id: UserControlId,
    ) -> UserControl {
        let now = time();

        let user_control = self.user_controls.get(&user).unwrap();

        let finalized_user_control = UserControl {
            owner: user_control.owner,
            user_control_id: Some(user_control_id),
            created_at: user_control.created_at,
            updated_at: now,
        };

        self.user_controls
            .insert(user, finalized_user_control.clone());

        finalized_user_control
    }

    pub fn get_user_control(&self, user: &UserId) -> Option<UserControl> {
        self.user_controls.get(user).cloned()
    }

    pub fn get_user_control_id(&self, user: &UserId) -> Option<UserControlId> {
        let user_control = self.user_controls.get(user).cloned();

        match user_control {
            Some(user_control) => user_control.user_control_id,
            None => None,
        }
    }

    pub fn get_user_ids(&self) -> Vec<UserId> {
        self.user_controls.keys().cloned().collect()
    }

    pub fn get_controllers(&self) -> Vec<ControllerId> {
        self.controllers.clone()
    }

    pub fn remove_user_control(&mut self, user: &UserId) {
        self.user_controls.remove(user);
    }

    pub fn add_controller(&mut self, controller_id: ControllerId) {
        self.controllers.push(controller_id);
    }

    pub fn remove_controller(&mut self, controller: ControllerId) {
        self.controllers.retain(|c| c != &controller);
    }

    pub fn latest_release(&self) -> Result<&Release, String> {
        self.releases.last().ok_or("No releases found!".to_string())
    }

    pub fn get_wasm_arg(&self, user: UserId) -> Result<WasmArg, String> {
        let wasm: Wasm = self
            .latest_release()?
            .get_wasm()
            .ok_or("No wasm found".to_string())?;

        let install_arg: Vec<u8> = Encode!(&UserControlArgs {
            owner: user.to_owned()
        })
        .map_err(|e| format!("Failed to encode install arg: {}!", e))?;

        Ok(WasmArg { wasm, install_arg })
    }
}

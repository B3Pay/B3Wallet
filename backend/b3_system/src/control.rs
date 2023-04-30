use std::collections::HashMap;

use crate::{
    canister::{create_canister_install_code, update_user_control_controllers, WasmArg},
    State, STATE,
};
use candid::{CandidType, Deserialize, Encode, Principal};
use ic_cdk::{api::time, trap};

pub type UserControlId = Principal;
pub type ControllerId = Principal;
pub type UserId = Principal;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserControl {
    pub user_control_id: Option<UserControlId>,
    pub created_at: u64,
    pub updated_at: u64,
    pub owner: UserId,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Controller {
    pub created_at: u64,
    pub updated_at: u64,
    pub expires_at: Option<u64>,
}

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct Release {
    pub wasm: Vec<u8>,
    pub version: Option<String>,
}

#[derive(CandidType)]
pub struct LoadRelease {
    pub total: usize,
    pub chunks: usize,
}

#[derive(CandidType)]
pub struct UserControlArgs {
    pub owner: UserId,
}

pub type Controllers = HashMap<ControllerId, Controller>;

pub const CREATE_USER_CANISTER_CYCLES: u128 = 1_000_000_000_000;

impl State {
    pub fn init_user_control(&mut self, user: &UserId) -> UserControl {
        let now = time();

        let mission_control = UserControl {
            owner: *user,
            user_control_id: None,
            created_at: now,
            updated_at: now,
        };

        self.user_controls.insert(*user, mission_control.clone());

        mission_control
    }

    pub fn add_user_control(
        &mut self,
        user: &UserId,
        user_control_id: UserControlId,
    ) -> UserControl {
        let now = time();

        // We know for sure that we have created an empty mission control center
        let user_control = self.user_controls.get(user).unwrap();

        let finalized_user_control = UserControl {
            owner: user_control.owner,
            user_control_id: Some(user_control_id),
            created_at: user_control.created_at,
            updated_at: now,
        };

        self.user_controls
            .insert(*user, finalized_user_control.clone());

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

    pub fn remove_user(&mut self, user: &UserId) {
        self.remove_controller(*user);
    }

    pub fn add_controller(&mut self, controller_id: ControllerId) {
        let now = time();

        let controller = Controller {
            created_at: now,
            updated_at: now,
            expires_at: None,
        };

        self.controllers.insert(controller_id, controller);
    }

    pub fn remove_controller(&mut self, controller: ControllerId) {
        self.controllers.remove(&controller);
    }

    pub fn update_release(&mut self, blob: &Vec<u8>, version: String) {
        let wasm = self
            .release
            .wasm
            .iter()
            .copied()
            .chain(blob.iter().copied())
            .collect();

        self.release.wasm = wasm;
        self.release.version = Some(version);
    }

    pub fn remove_release(&mut self) {
        self.release.wasm = Vec::new();
        self.release.version = None;
    }
}

pub async fn new_user_control(user: &UserId, system: &Principal) -> Result<UserControl, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let user_control = state.get_user_control(user);

        match user_control {
            None => {
                state.init_user_control(user);
            }
            _ => trap("User already has a control center."),
        }
    });

    let wasm_arg = user_wasm_arg(user);

    let user_control_id = create_canister_install_code(
        Vec::from([*system, *user]),
        &wasm_arg,
        CREATE_USER_CANISTER_CYCLES,
    )
    .await;

    match user_control_id {
        Err(e) => {
            // We delete the pending empty mission control center from the list - e.g. this can happens if manager is out of cycles and user would be blocked
            STATE.with(|state| state.borrow_mut().remove_user(user));
            Err(["Canister cannot be initialized.", &e].join(""))
        }
        Ok(user_control_id) => {
            let user_control = STATE.with(|state| {
                let mut state = state.borrow_mut();
                state.add_user_control(user, user_control_id)
            });

            update_user_control_controllers(&user_control_id, user).await?;

            Ok(user_control)
        }
    }
}

pub fn user_wasm_arg(user: &UserId) -> WasmArg {
    let wasm: Vec<u8> = STATE.with(|state| state.borrow().release.wasm.clone());

    let install_arg: Vec<u8> = Encode!(&UserControlArgs { owner: *user }).unwrap();
    WasmArg { wasm, install_arg }
}

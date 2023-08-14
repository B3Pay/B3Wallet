use crate::types::RoleMap;
use b3_utils::{nonce::Nonce, types::RoleId};
use candid::{CandidType, Deserialize};

use super::Role;

#[derive(CandidType, Deserialize, PartialEq, Debug, Clone)]
pub struct RoleState {
    next_role_id: Nonce,
    roles: RoleMap,
}

impl Default for RoleState {
    fn default() -> Self {
        RoleState {
            next_role_id: Nonce::default(),
            roles: RoleMap::new(),
        }
    }
}

impl RoleState {
    pub fn init(&mut self, roles: RoleMap) {
        self.roles = roles;
    }

    pub fn add(&mut self, role: Role) {
        let role_id = self.next_role_id.next();

        self.roles.insert(role_id, role);
    }

    pub fn remove(&mut self, role_id: &RoleId) {
        self.roles.remove(role_id);
    }

    pub fn role(&self, role_id: &RoleId) -> Option<&Role> {
        self.roles.get(role_id)
    }

    pub fn role_mut(&mut self, role_id: &RoleId) -> Option<&mut Role> {
        self.roles.get_mut(role_id)
    }

    pub fn roles(&self) -> &RoleMap {
        &self.roles
    }

    pub fn roles_mut(&mut self) -> &mut RoleMap {
        &mut self.roles
    }
}

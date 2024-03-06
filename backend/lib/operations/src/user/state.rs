use crate::{error::OperationError, types::UserMap};
use b3_utils::principal::StoredPrincipal;
use candid::{CandidType, Deserialize};

use super::User;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserState(UserMap);

impl Default for UserState {
    fn default() -> Self {
        UserState(UserMap::new())
    }
}

impl UserState {
    pub fn init(&mut self, signers: UserMap) {
        self.0 = signers;
    }

    pub fn add(&mut self, user_id: StoredPrincipal, user: User) {
        self.0.insert(user_id, user);
    }

    pub fn remove(&mut self, user_id: &StoredPrincipal) {
        self.0.remove(user_id);
    }

    pub fn user(&self, user_id: &StoredPrincipal) -> Result<&User, OperationError> {
        self.0
            .get(user_id)
            .ok_or(OperationError::UserNotFound(user_id.clone()))
    }

    pub fn user_mut(&mut self, user_id: &StoredPrincipal) -> Result<&mut User, OperationError> {
        self.0
            .get_mut(user_id)
            .ok_or(OperationError::UserNotFound(user_id.clone()))
    }

    pub fn contains(&self, user_id: &StoredPrincipal) -> bool {
        self.0.contains_key(user_id)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&StoredPrincipal, &User)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&StoredPrincipal, &mut User)> {
        self.0.iter_mut()
    }

    pub fn get_users(&self) -> UserMap {
        self.0.clone()
    }

    pub fn set_users(&mut self, users: UserMap) {
        self.0 = users;
    }

    pub fn users(&self) -> &UserMap {
        &self.0
    }

    pub fn users_mut(&mut self) -> &mut UserMap {
        &mut self.0
    }
}

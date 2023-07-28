use crate::{error::OperationError, types::UserMap};
use b3_utils::types::UserId;
use candid::{CandidType, Deserialize};

use super::User;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserState {
    users: UserMap,
}

impl Default for UserState {
    fn default() -> Self {
        UserState {
            users: UserMap::new(),
        }
    }
}

impl UserState {
    pub fn init_users(&mut self, signers: UserMap) {
        self.users = signers;
    }

    pub fn add_user(&mut self, user_id: UserId, user: User) {
        self.users.insert(user_id, user);
    }

    pub fn remove_user(&mut self, user_id: &UserId) {
        self.users.remove(user_id);
    }

    pub fn user(&self, user_id: &UserId) -> Result<&User, OperationError> {
        self.users
            .get(user_id)
            .ok_or(OperationError::UserNotFound(user_id.to_string()))
    }

    pub fn user_mut(&mut self, user_id: &UserId) -> Result<&mut User, OperationError> {
        self.users
            .get_mut(user_id)
            .ok_or(OperationError::UserNotFound(user_id.to_string()))
    }

    pub fn users(&self) -> &UserMap {
        &self.users
    }

    pub fn users_mut(&mut self) -> &mut UserMap {
        &mut self.users
    }
}

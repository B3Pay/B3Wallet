use b3_utils::types::{CanisterId, CanisterIds, UserId};

pub mod error;
pub mod store;
pub mod test;
pub mod types;
pub mod user;
use error::UserSystemError;
use store::UserMap;
use user::User;

use self::types::{UserStates, Users};

pub struct UserState {
    pub users: UserMap,
}

impl UserState {
    // App
    pub fn init_user(&mut self, user: UserId) -> Result<User, UserSystemError> {
        if let Some(user_state) = self.users.get(&user) {
            if !user_state.canisters().is_empty() {
                return Err(UserSystemError::UserAlreadyExists);
            }
        }

        let user_state = User::new(None);

        self.users.insert(user, user_state.clone());

        Ok(user_state)
    }

    pub fn get_or_init_user(
        &mut self,
        user: UserId,
        opt_canister_id: Option<CanisterId>,
    ) -> Result<User, UserSystemError> {
        if let Some(mut states) = self.users.get(&user) {
            let mut user_state = states.update_rate()?;

            if let Some(canister_id) = opt_canister_id {
                user_state.add_canister(canister_id);
            }

            return Ok(user_state);
        }

        let user_state = User::new(opt_canister_id);

        self.users.insert(user, user_state.clone());

        Ok(user_state)
    }

    pub fn add_user(&mut self, user: UserId, user_state: User) {
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

    pub fn user_state(&self, user_id: UserId) -> Result<User, UserSystemError> {
        self.users
            .get(&user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(|state| state.clone())
    }

    pub fn user_states(&self) -> UserStates {
        self.users.iter().map(|(_, v)| v.clone()).collect()
    }

    pub fn number_of_users(&self) -> u64 {
        self.users.len()
    }
}

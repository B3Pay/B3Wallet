use super::{
    error::UserSystemError,
    types::{UserStates, UserView, UserViews, Users},
    User,
};
use b3_utils::{
    memory::types::DefaultStableBTreeMap,
    types::{CanisterId, CanisterIds, UserId},
};

pub type UserMap = DefaultStableBTreeMap<UserId, User>;

pub struct UserState {
    pub users: UserMap,
}

// Write to the UserState struct
impl UserState {
    pub fn initialize_user(&mut self, user_id: UserId) -> Result<User, UserSystemError> {
        if let Some(user_state) = self.users.get(&user_id) {
            if !user_state.canisters().is_empty() {
                return Err(UserSystemError::UserAlreadyExists);
            }

            return Ok(user_state);
        }

        let user_state = User::new(None);

        self.users.insert(user_id, user_state.clone());

        Ok(user_state)
    }

    pub fn get_user_or_initialize(
        &mut self,
        user_id: UserId,
        canister_id: Option<CanisterId>,
    ) -> Result<User, UserSystemError> {
        if let Some(mut user) = self.users.get(&user_id) {
            let mut user_state = user.update_rate()?;

            if let Some(canister_id) = canister_id {
                user_state.add_canister(canister_id);
            }

            return Ok(user_state);
        }

        let user_state = User::new(canister_id);

        self.users.insert(user_id, user_state.clone());

        Ok(user_state)
    }

    pub fn add(&mut self, user: UserId, user_state: User) {
        self.users.insert(user, user_state);
    }

    pub fn remove(&mut self, user: &UserId) {
        self.users.remove(user);
    }
}

// Read from the UserState struct
impl UserState {
    pub fn user_view(&self, user_id: &UserId) -> Result<UserView, UserSystemError> {
        self.users
            .get(user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(|state| state.view())
    }

    pub fn user(&self, user_id: &UserId) -> Option<User> {
        self.users.get(user_id)
    }

    pub fn users_view(&self) -> UserViews {
        self.users.iter().map(|(_, v)| v.view()).collect()
    }

    pub fn users(&self) -> UserStates {
        self.users.iter().map(|(_, v)| v.clone()).collect()
    }

    pub fn canister_ids(&self) -> CanisterIds {
        self.users
            .iter()
            .map(|(_, v)| v.canisters())
            .flatten()
            .collect()
    }

    pub fn user_ids(&self) -> Users {
        self.users.iter().map(|(k, _)| k).collect()
    }

    pub fn number_of_users(&self) -> u64 {
        self.users.len()
    }
}

use super::{
    error::UserSystemError,
    store::{with_user, with_users_mut},
    types::{CreateUserArgs, UserView},
    User,
};
use b3_utils::types::{CanisterId, CanisterIds, UserId};

#[cfg(test)]
use b3_utils::mocks::id_mock as ic_cdk_caller;
#[cfg(not(test))]
use ic_cdk::api::caller as ic_cdk_caller;

pub struct WriteUserState(pub UserId);

impl WriteUserState {
    pub fn update(&mut self, app_args: CreateUserArgs) -> Result<User, UserSystemError> {
        with_users_mut(|users| {
            let mut user = users.get(&self.0).ok_or(UserSystemError::UserNotFound)?;

            user.update(app_args)?;

            users.insert(self.0.clone(), user.clone());

            Ok(user.clone())
        })
    }

    pub fn add_canister(&mut self, canister_id: CanisterId) -> Result<User, UserSystemError> {
        with_users_mut(|users| {
            let mut user = users.get(&self.0).ok_or(UserSystemError::UserNotFound)?;

            user.add_canister(canister_id);

            users.insert(self.0.clone(), user.clone());

            Ok(user.clone())
        })
    }

    pub fn remove_canister(&mut self, canister_id: CanisterId) -> Result<User, UserSystemError> {
        with_users_mut(|users| {
            let mut user = users.get(&self.0).ok_or(UserSystemError::UserNotFound)?;

            user.remove_canister(canister_id)?;

            users.insert(self.0.clone(), user.clone());

            Ok(user.clone())
        })
    }

    pub fn remove(&mut self) -> Result<(), UserSystemError> {
        with_users_mut(|users| {
            users.remove(&self.0).ok_or(UserSystemError::UserNotFound)?;

            Ok(())
        })
    }
}

pub struct ReadUserState(pub UserId);

impl ReadUserState {
    pub fn user_view(&self) -> Result<UserView, UserSystemError> {
        with_user(&self.0, |user| user.view())
    }

    pub fn user(&self) -> Result<User, UserSystemError> {
        with_user(&self.0, |user| user.clone())
    }

    pub fn canisters(&self) -> Result<CanisterIds, UserSystemError> {
        with_user(&self.0, |user| user.canisters())
    }

    pub fn verify_canister(&self, canister_id: &CanisterId) -> Result<(), UserSystemError> {
        with_user(&self.0, |user| user.verify_canister(canister_id))?
    }
}

pub struct UserState;

impl UserState {
    pub fn create(app_args: CreateUserArgs) -> Result<User, UserSystemError> {
        let user = User::new(app_args);
        let user_id = ic_cdk_caller().into();

        with_users_mut(|users| {
            if users.contains_key(&user_id) {
                return Err(UserSystemError::UserAlreadyExists);
            }

            users.insert(user_id, user.clone());

            Ok(user)
        })
    }

    pub fn write(user_id: UserId) -> WriteUserState {
        WriteUserState(user_id)
    }

    pub fn read(user_id: UserId) -> ReadUserState {
        ReadUserState(user_id)
    }
}

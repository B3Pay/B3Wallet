use b3_utils::types::{CanisterId, CanisterIds, UserId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::user::user::User;

pub type UserStates = Vec<User>;

pub type Features = Vec<String>;
pub type Users = Vec<UserId>;

#[derive(CandidType, Deserialize, Serialize)]
pub enum UserStatus {
    Registered,
    Unregistered,
    SingleCanister(CanisterId),
    MultipleCanister(CanisterIds),
}

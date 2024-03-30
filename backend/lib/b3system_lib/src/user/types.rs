use b3_utils::{
    ledger::Metadata,
    principal::StoredPrincipal,
    types::{CanisterId, CanisterIds},
    NanoTimeStamp,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::user::user::User;

pub type Features = Vec<String>;
pub type Users = Vec<StoredPrincipal>;

pub type UserStates = Vec<User>;

#[derive(CandidType, Deserialize, Serialize)]
pub enum UserStatus {
    Registered,
    Unregistered,
    Applications(CanisterIds),
}

pub type UserViews = Vec<UserView>;

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserView {
    pub canisters: Vec<CanisterId>,
    pub created_at: NanoTimeStamp,
    pub updated_at: NanoTimeStamp,
    pub metadata: Metadata,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateUserArgs {
    pub canister_id: Option<CanisterId>,
    pub metadata: Metadata,
}

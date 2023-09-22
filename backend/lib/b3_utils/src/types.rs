use std::collections::HashMap;

use crate::nonce::Nonce;
use candid::Principal;

pub type ControllerId = Principal;
pub type ControllerIds = Vec<ControllerId>;

pub type CanisterId = Principal;
pub type UserId = Principal;

pub type RoleId = Nonce;

pub type OperationId = u64;
pub type Deadline = u64;

pub type Metadata = HashMap<String, String>;

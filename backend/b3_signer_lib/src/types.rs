use crate::{
    account::SignerAccount, allowance::SignerAllowance, request::sign::SignRequest,
    signer::SignerUser,
};
use b3_helper::types::{CanisterId, SignerId};

use std::collections::{BTreeMap, HashMap};

pub type Metadata = HashMap<String, String>;

pub type Accounts = BTreeMap<String, SignerAccount>;

pub type CanisterAllowances = HashMap<CanisterId, SignerAllowance>;

pub type SignerUsers = HashMap<SignerId, SignerUser>;

pub type SignRequests = Vec<SignRequest>;

pub type RequestId = String;

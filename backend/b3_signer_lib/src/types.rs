use crate::{account::SignerAccount, allowance::SignerAllowance, request::EvmSignRequest};
use b3_helper::types::CanisterId;

use std::collections::{BTreeMap, HashMap};

pub type Metadata = HashMap<String, String>;

pub type Accounts = BTreeMap<String, SignerAccount>;

pub type CanisterAllowances = HashMap<CanisterId, SignerAllowance>;

pub type CanisterRequests = HashMap<CanisterId, EvmSignRequest>;

use crate::{account::WalletAccount, request::Request, signer::Signer};
use b3_helper::types::SignerId;

use std::collections::{BTreeMap, HashMap};

pub type RequestId = usize;

pub type AccountId = String;

pub type ConfirmedRequests = Vec<Request>;

pub type Metadata = HashMap<String, String>;

pub type SignerMap = HashMap<SignerId, Signer>;

pub type RequestMap = HashMap<RequestId, Request>;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

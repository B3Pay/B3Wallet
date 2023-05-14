use crate::{account::WalletAccount, request::Request, signed::SignedTransaction, signer::Signer};
use b3_helper::types::SignerId;

use std::collections::{BTreeMap, HashMap};

pub type RequestId = usize;

pub type AccountId = String;

pub type Metadata = HashMap<String, String>;

pub type ConfirmedMap = HashMap<RequestId, SignedTransaction>;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

pub type SignerMap = HashMap<SignerId, Signer>;

pub type Requests = Vec<Request>;

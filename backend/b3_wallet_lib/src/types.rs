use crate::{
    account::WalletAccount, confirmed::ConfirmedRequest, request::PendingRequest, signer::Signer,
};
use b3_helper::types::SignerId;
use std::collections::{BTreeMap, HashMap};

pub type RequestId = usize;

pub type Deadline = u64;

pub type AccountId = String;

pub type SignedMessage = Vec<u8>;

pub type Metadata = HashMap<String, String>;

pub type SignerMap = HashMap<SignerId, Signer>;

pub type WalletAccountMap = BTreeMap<String, WalletAccount>;

pub type PendingRequestMap = HashMap<RequestId, PendingRequest>;

pub type ConfirmedRequestMap = HashMap<RequestId, ConfirmedRequest>;

pub type PendingRequestList = Vec<PendingRequest>;

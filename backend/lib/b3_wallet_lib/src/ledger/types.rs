use std::collections::BTreeMap;

use super::btc::network::BtcNetwork;
use b3_helper_lib::types::{CanisterId, Subaccount};
use bitcoin::{AddressType, OutPoint, Transaction, TxIn, TxOut, Txid};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

pub type ChainId = u64;

pub type AddressMap = BTreeMap<Chains, String>;

pub type EcdsaPublicKey = Vec<u8>;

pub type BtcAddressType = AddressType;

pub type BtcTransaction = Transaction;

pub type BtcTxIn = TxIn;

pub type BtcTxOut = TxOut;

pub type BtcTxId = Txid;

pub type BtcOutPoint = OutPoint;

#[derive(CandidType, Clone, Deserialize)]
pub struct Ledger {
    pub keys: Keys,
    pub subaccount: Subaccount,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Keys {
    pub ecdsa: Option<EcdsaPublicKey>,
    pub subaccount: Subaccount,
    pub addresses: AddressMap,
}

#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Chains {
    ICRC(CanisterId),
    BTC(BtcNetwork),
    EVM(ChainId),
    ICP,
}

#[derive(CandidType, Serialize)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct ECDSAPublicKeyResponse {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct SignWithECDSAResponse {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize)]
pub struct ECDSAPublicKeyArgs {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct SignWithECDSAArgs {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Clone, Deserialize)]
pub struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Clone, Deserialize)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

use std::collections::HashMap;

use bitcoin::{AddressType, OutPoint, Transaction, TxIn, TxOut, Txid};
use ic_cdk::{
    api::management_canister::bitcoin::{BitcoinNetwork, Utxo},
    export::{
        candid::CandidType,
        serde::{Deserialize, Serialize},
    },
};

use b3_helper::types::CanisterId;

use super::network::Network;

pub type ChainId = u64;

pub type AddressMap = HashMap<Network, String>;

pub type Ecdsa = Vec<u8>;

pub type BtcNetwork = BitcoinNetwork;

pub type BtcAddressType = AddressType;

pub type BtcTransaction = Transaction;

pub type BtcTxIn = TxIn;

pub type BtcTxOut = TxOut;

pub type BtcTxId = Txid;

pub type BtcUtxo = Utxo;

pub type BtcOutPoint = OutPoint;

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

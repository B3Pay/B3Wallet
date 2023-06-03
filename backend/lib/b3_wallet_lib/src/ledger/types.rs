use super::{btc::network::BtcNetwork, icrc::account::ICRCAccount};
use crate::error::WalletError;
use async_trait::async_trait;
use b3_helper_lib::{
    constants::{CANISTER_TRANSFER_MEMO, IC_TRANSACTION_FEE_ICP},
    types::{AccountIdentifier, CanisterId, Memo, Subaccount, Timestamp, Tokens},
};
use bitcoin::{AddressType, OutPoint, Transaction, TxIn, TxOut, Txid};
use candid::Nat;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::collections::BTreeMap;

pub type ChainId = u64;

pub type Balance = Nat;

pub type ICRCFee = Nat;

pub type ICRCMemo = Vec<u8>;

pub type ICRCTimestamp = u64;

pub type EcdsaPublicKey = Vec<u8>;

pub type BtcAddressType = AddressType;

pub type BtcTransaction = Transaction;

pub type BtcTxIn = TxIn;

pub type BtcTxOut = TxOut;

pub type BtcTxId = Txid;

pub type BtcOutPoint = OutPoint;

pub type ChainMap = BTreeMap<ChainType, Chain>;

#[derive(CandidType, PartialEq, Eq, PartialOrd, Ord, Deserialize, Clone)]
pub enum ChainType {
    ICRC(CanisterId),
    BTC(BtcNetwork),
    EVM(ChainId),
    ICP,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Ledger {
    pub ecdsa: Option<EcdsaPublicKey>,
    pub subaccount: Subaccount,
    pub chains: ChainMap,
}

#[async_trait]
#[enum_dispatch]
pub trait ChainTrait {
    async fn balance(&self) -> Result<Balance, WalletError>;
}

#[enum_dispatch(ChainTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Chain {
    #[serde(rename = "icrc")]
    ICRC,
    #[serde(rename = "btc")]
    BTC,
    #[serde(rename = "evm")]
    EVM,
    #[serde(rename = "icp")]
    ICP,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::ICP(ICP::new(AccountIdentifier::default()))
    }
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ICP {
    pub identifier: AccountIdentifier,
    pub memo: Memo,
    pub fee: Tokens,
    pub created_at_time: Option<Timestamp>,
}

impl ICP {
    pub fn new(identifier: AccountIdentifier) -> Self {
        ICP {
            identifier,
            memo: CANISTER_TRANSFER_MEMO,
            fee: IC_TRANSACTION_FEE_ICP,
            created_at_time: None,
        }
    }
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ICRC {
    pub canister_id: CanisterId,
    pub account: ICRCAccount,
    pub fee: ICRCFee,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
}

impl ICRC {
    pub fn new(canister_id: CanisterId, subaccount: Subaccount, fee: ICRCFee) -> Self {
        ICRC {
            canister_id,
            account: subaccount.into(),
            fee,
            memo: None,
            created_at_time: None,
        }
    }
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct BTC {
    pub btc_network: BtcNetwork,
    pub address: String, // Added address field
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct EVM {
    pub chain_id: ChainId,
    pub address: String,
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

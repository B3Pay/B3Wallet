use super::{
    btc::network::BtcNetwork,
    chain::Chain,
    ckbtc::ckbtc::CKBTC,
    icrc::types::{TxIndex, ICRC},
};
use crate::error::WalletError;
use async_trait::async_trait;
use b3_helper_lib::{
    constants::{CANISTER_TRANSFER_MEMO, IC_TRANSACTION_FEE_ICP},
    subaccount::Subaccount,
    tokens::Tokens,
    types::{CanisterId, Memo, Timestamp, TransferResult},
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

pub type EcdsaPublicKey = Vec<u8>;

pub type BtcAddressType = AddressType;

pub type BtcTransaction = Transaction;

pub type BtcTxIn = TxIn;

pub type BtcTxOut = TxOut;

pub type BtcTxId = Txid;

pub type BtcOutPoint = OutPoint;

pub type ChainMap = BTreeMap<ChainEnum, Chain>;

pub type AddressMap = BTreeMap<ChainEnum, String>;

#[derive(CandidType, PartialEq, Eq, PartialOrd, Ord, Deserialize, Clone)]
pub enum ChainEnum {
    CKBTC(BtcNetwork),
    ICRC(CanisterId),
    BTC(BtcNetwork),
    EVM(ChainId),
    ICP,
}

impl ChainEnum {
    pub fn is_icrc(&self) -> bool {
        matches!(self, ChainEnum::ICRC(_))
    }

    pub fn is_btc(&self) -> bool {
        matches!(self, ChainEnum::BTC(_))
    }

    pub fn is_evm(&self) -> bool {
        matches!(self, ChainEnum::EVM(_))
    }

    pub fn is_icp(&self) -> bool {
        matches!(self, ChainEnum::ICP)
    }
}

#[async_trait]
#[enum_dispatch]
pub trait ChainTrait {
    fn address(&self) -> String;
    async fn balance(&self) -> Result<Balance, WalletError>;
    async fn send(&self, to: String, amount: u64) -> Result<SendResult, WalletError>;
    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        fee: Option<u64>,
        memo: Option<String>,
    ) -> Result<SendResult, WalletError>;
}

pub enum SendResult {
    ICP(TransferResult),
    CKBTC(TxIndex),
    ICRC(TxIndex),
    BTC(BtcTxId),
    EVM,
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ICP {
    pub subaccount: Subaccount,
    pub memo: Memo,
    pub fee: Tokens,
    pub created_at_time: Option<Timestamp>,
}

impl ICP {
    pub fn new(subaccount: Subaccount) -> Self {
        ICP {
            subaccount,
            memo: CANISTER_TRANSFER_MEMO,
            fee: IC_TRANSACTION_FEE_ICP,
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

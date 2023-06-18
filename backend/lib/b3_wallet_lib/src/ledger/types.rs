use super::{btc::network::BtcNetwork, chain::Chain, ckbtc::types::BtcTxId, icrc::types::TxIndex};
use b3_helper_lib::types::{BlockIndex, CanisterId};
use bitcoin::{AddressType, OutPoint, Transaction, TxIn, TxOut};
use candid::Nat;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::collections::BTreeMap;
use std::fmt;

pub type ChainId = u64;

pub type Balance = Nat;

pub type BtcAddressType = AddressType;

pub type BtcTransaction = Transaction;

pub type BtcTxIn = TxIn;

pub type BtcTxOut = TxOut;

pub type BtcOutPoint = OutPoint;

pub type ChainMap = BTreeMap<ChainEnum, Chain>;

pub type AddressMap = BTreeMap<ChainEnum, String>;

#[derive(CandidType, PartialEq, Serialize, Eq, PartialOrd, Ord, Deserialize, Clone, Debug)]
pub struct BtcPending {
    pub txid: BtcTxId,
    pub account: String,
}

#[derive(CandidType, PartialEq, Serialize, Eq, PartialOrd, Ord, Deserialize, Clone, Debug)]
pub struct IcpPending {
    pub block_index: BlockIndex,
    pub canister_id: String,
}

#[derive(CandidType, PartialEq, Serialize, Eq, PartialOrd, Ord, Deserialize, Clone, Debug)]
pub struct EvmPending {
    pub block_index: BlockIndex,
}

#[derive(CandidType, PartialEq, Serialize, Eq, PartialOrd, Ord, Deserialize, Clone, Debug)]
pub struct CkbtcPending {
    pub txid: Option<TxIndex>,
    pub block_index: BlockIndex,
}

#[derive(CandidType, PartialEq, Serialize, Eq, PartialOrd, Ord, Deserialize, Clone, Debug)]
pub struct IcrcPending {
    pub tx_index: TxIndex,
    pub block_index: BlockIndex,
}

pub type Pendings = Vec<PendingEnum>;

#[enum_dispatch(PendingTrait)]
#[derive(CandidType, PartialEq, Serialize, Eq, PartialOrd, Ord, Deserialize, Clone, Debug)]
pub enum PendingEnum {
    CKBTC(CkbtcPending),
    ICRC(IcrcPending),
    BTC(BtcPending),
    EVM(EvmPending),
    ICP(IcpPending),
}

impl PendingEnum {
    pub fn new_ckbtc(block_index: BlockIndex, txid: Option<TxIndex>) -> Self {
        PendingEnum::CKBTC(CkbtcPending { txid, block_index })
    }

    pub fn new_icrc(block_index: BlockIndex, tx_index: TxIndex) -> Self {
        PendingEnum::ICRC(IcrcPending {
            tx_index,
            block_index,
        })
    }

    pub fn new_btc(txid: BtcTxId, account: String) -> Self {
        PendingEnum::BTC(BtcPending { txid, account })
    }

    pub fn new_evm(block_index: BlockIndex) -> Self {
        PendingEnum::EVM(EvmPending { block_index })
    }

    pub fn new_icp(block_index: BlockIndex, canister_id: String) -> Self {
        PendingEnum::ICP(IcpPending {
            block_index,
            canister_id,
        })
    }
}

#[enum_dispatch]
pub trait PendingTrait {}

#[derive(CandidType, PartialEq, Eq, Debug, PartialOrd, Ord, Deserialize, Clone)]
pub enum ChainEnum {
    CKBTC(BtcNetwork),
    ICRC(CanisterId),
    BTC(BtcNetwork),
    EVM(ChainId),
    ICP,
}

impl fmt::Display for ChainEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainEnum::CKBTC(network) => write!(f, "CKBTC({})", network),
            ChainEnum::ICRC(canister_id) => write!(f, "ICRC({})", canister_id),
            ChainEnum::BTC(network) => write!(f, "BTC({})", network),
            ChainEnum::EVM(chain_id) => write!(f, "EVM({})", chain_id),
            ChainEnum::ICP => write!(f, "ICP"),
        }
    }
}

impl ChainEnum {
    pub fn is_ckbtc(&self) -> bool {
        matches!(self, ChainEnum::CKBTC(_))
    }

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

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum SendResult {
    ICP(BlockIndex),
    CKBTC(TxIndex),
    ICRC(TxIndex),
    BTC(BtcTxId),
    EVM,
}

impl fmt::Display for SendResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendResult::ICP(result) => write!(f, "ICP({:?})", result),
            SendResult::CKBTC(tx_index) => write!(f, "CKBTC({})", tx_index),
            SendResult::ICRC(tx_index) => write!(f, "ICRC({})", tx_index),
            SendResult::BTC(txid) => write!(f, "BTC({})", txid),
            SendResult::EVM => write!(f, "EVM"),
        }
    }
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

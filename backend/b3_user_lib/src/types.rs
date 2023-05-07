use std::collections::HashMap;

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

use crate::{
    allowance::{Allowance, CanisterId},
    request::SignRequest,
};

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct UserControlArgs {
    pub owner: Principal,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum Network {
    Bitcoin(BitcoinNetwork),
    Ethereum,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bitcoin(network) => match network {
                BitcoinNetwork::Mainnet => write!(f, "bitcoin"),
                BitcoinNetwork::Testnet => write!(f, "bitcoin_testnet"),
                BitcoinNetwork::Regtest => write!(f, "bitcoin_regtest"),
            },
            Self::Ethereum => write!(f, "ethereum"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

impl std::fmt::Display for BitcoinNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Testnet => write!(f, "testnet"),
            Self::Regtest => write!(f, "regtest"),
        }
    }
}

pub type CanisterHashMap = HashMap<CanisterId, Allowance>;
pub type RequestHashMap = HashMap<CanisterId, SignRequest>;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct CanisterStatus {
    pub id: Principal,
    pub version: String,
    pub status: CanisterStatusResponse,
    pub status_at: u64,
}

#[derive(Debug, Clone, CandidType, Default, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Pending,
    Success,
    Failed,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct IcpXdrConversionRate {
    pub timestamp_seconds: u64,
    pub xdr_permyriad_per_icp: u64,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct IcpXdrConversionRateCertifiedResponse {
    pub data: IcpXdrConversionRate,
    pub hash_tree: Vec<u8>,
    pub certificate: Vec<u8>,
}

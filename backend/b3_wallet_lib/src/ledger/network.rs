use super::types::ChainId;
use crate::error::WalletError;
use bitcoin::Network as BitcoinCrateNetwork;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Serialize;
use std::fmt;

#[derive(CandidType, Clone, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum Network {
    SNS(String),
    BTC(BtcNetwork),
    EVM(ChainId),
    ICP,
}

/// Bitcoin Network.
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub enum BtcNetwork {
    /// Mainnet.
    #[serde(rename = "mainnet")]
    Mainnet,
    /// Testnet.
    #[serde(rename = "testnet")]
    Testnet,
    /// Regtest.
    ///
    /// This is only available when developing with local replica.
    #[serde(rename = "regtest")]
    Regtest,
}

impl Default for BtcNetwork {
    fn default() -> Self {
        Self::Regtest
    }
}

impl From<BitcoinNetwork> for BtcNetwork {
    fn from(network: BitcoinNetwork) -> Self {
        match network {
            BitcoinNetwork::Mainnet => BtcNetwork::Mainnet,
            BitcoinNetwork::Testnet => BtcNetwork::Testnet,
            BitcoinNetwork::Regtest => BtcNetwork::Regtest,
        }
    }
}

impl From<BitcoinCrateNetwork> for BtcNetwork {
    fn from(network: BitcoinCrateNetwork) -> Self {
        match network {
            BitcoinCrateNetwork::Bitcoin => BtcNetwork::Mainnet,
            BitcoinCrateNetwork::Testnet => BtcNetwork::Testnet,
            BitcoinCrateNetwork::Regtest => BtcNetwork::Regtest,
            _ => panic!("Invalid network"),
        }
    }
}

impl From<BtcNetwork> for BitcoinCrateNetwork {
    fn from(network: BtcNetwork) -> Self {
        match network {
            BtcNetwork::Mainnet => BitcoinCrateNetwork::Bitcoin,
            BtcNetwork::Testnet => BitcoinCrateNetwork::Testnet,
            BtcNetwork::Regtest => BitcoinCrateNetwork::Regtest,
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BTC(network) => match network {
                BtcNetwork::Mainnet => write!(f, "btc"),
                BtcNetwork::Testnet => write!(f, "btc_testnet"),
                BtcNetwork::Regtest => write!(f, "btc_regtest"),
            },
            Self::EVM(chain_id) => write!(f, "evm_{}", chain_id),
            Self::SNS(token) => write!(f, "sns_{}", token),
            Self::ICP => write!(f, "icp"),
        }
    }
}

impl Network {
    pub fn from_str(network: &str) -> Result<Self, WalletError> {
        match network {
            "btc" => Ok(Self::BTC(BtcNetwork::Mainnet)),
            "btc_testnet" => Ok(Self::BTC(BtcNetwork::Testnet)),
            "btc_regtest" => Ok(Self::BTC(BtcNetwork::Regtest)),
            "icp" => Ok(Self::ICP),
            _ => {
                if network.starts_with("evm_") {
                    let chain_id = network
                        .strip_prefix("evm_")
                        .ok_or(WalletError::InvalidNetwork)?;
                    let chain_id = chain_id
                        .parse::<ChainId>()
                        .map_err(|_| WalletError::InvalidNetwork)?;

                    return Ok(Self::EVM(chain_id));
                } else if network.starts_with("sns_") {
                    let token = network
                        .strip_prefix("sns_")
                        .ok_or(WalletError::InvalidNetwork)?;

                    return Ok(Self::SNS(token.to_string()));
                } else {
                    return Err(WalletError::InvalidNetwork);
                }
            }
        }
    }
}

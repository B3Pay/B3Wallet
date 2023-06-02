use super::{btc::network::BtcNetwork, types::ChainId};
use crate::error::WalletError;
use b3_helper_lib::types::CanisterId;
use candid::{CandidType, Deserialize};
use std::fmt;

#[derive(CandidType, Clone, Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Chains {
    ICRC(CanisterId),
    BTC(BtcNetwork),
    EVM(ChainId),
    ICP,
}

impl fmt::Display for Chains {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BTC(network) => match network {
                BtcNetwork::Mainnet => write!(f, "btc"),
                BtcNetwork::Testnet => write!(f, "btc_testnet"),
                BtcNetwork::Regtest => write!(f, "btc_regtest"),
            },
            Self::EVM(chain_id) => write!(f, "evm_{}", chain_id),
            Self::ICRC(token) => write!(f, "sns_{}", token),
            Self::ICP => write!(f, "icp"),
        }
    }
}

impl Chains {
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

                    let canister_id =
                        CanisterId::from_text(token).map_err(|_| WalletError::InvalidToken)?;

                    return Ok(Self::ICRC(canister_id));
                } else {
                    return Err(WalletError::InvalidNetwork);
                }
            }
        }
    }
}

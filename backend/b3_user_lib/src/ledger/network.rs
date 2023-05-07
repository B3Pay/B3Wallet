use candid::{CandidType, Deserialize};

pub type ChainId = u64;

#[derive(CandidType, Clone, Deserialize)]
pub enum Network {
    SNS(String),
    BTC(BitcoinNetwork),
    EVM(ChainId),
    ICP,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BTC(network) => match network {
                BitcoinNetwork::Mainnet => write!(f, "btc"),
                BitcoinNetwork::Testnet => write!(f, "btc_testnet"),
                BitcoinNetwork::Regtest => write!(f, "btc_regtest"),
            },
            Self::EVM(chain_id) => write!(f, "evm_{}", chain_id),
            Self::SNS(token) => write!(f, "{}", token),
            Self::ICP => write!(f, "icp"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize)]
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

use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::{fmt, str::FromStr};

use crate::error::HelperError;

#[derive(CandidType, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum ReleaseName {
    Custom(String),
    #[serde(rename = "b3_wallet")]
    B3Wallet,
    #[serde(rename = "b3_basic_wallet")]
    B3SimpleWallet,
    #[serde(rename = "b3_multi_sig_wallet")]
    B3MultiSigWallet,
}

impl fmt::Display for ReleaseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseName::B3Wallet => write!(f, "b3_wallet"),
            ReleaseName::B3SimpleWallet => write!(f, "b3_basic_wallet"),
            ReleaseName::B3MultiSigWallet => write!(f, "b3_multi_sig_wallet"),
            ReleaseName::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl FromStr for ReleaseName {
    type Err = HelperError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "b3_wallet" => Ok(ReleaseName::B3Wallet),
            "b3_basic_wallet" => Ok(ReleaseName::B3SimpleWallet),
            "b3_multi_sig_wallet" => Ok(ReleaseName::B3MultiSigWallet),
            name if name.starts_with("custom_") => Ok(ReleaseName::Custom(name.to_string())),
            _ => Err(HelperError::InvalidReleaseName(name.to_string())),
        }
    }
}

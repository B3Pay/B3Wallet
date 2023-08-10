use candid::CandidType;
use serde::Deserialize;
use std::{fmt, str::FromStr};

use crate::error::HelperError;

#[derive(CandidType, PartialEq, Eq, Hash, Deserialize, Clone)]
pub enum ReleaseNames {
    Custom(String),
    #[serde(rename = "b3_wallet")]
    B3Wallet,
    #[serde(rename = "b3_basic_wallet")]
    B3SimpleWallet,
    #[serde(rename = "b3_multi_sig_wallet")]
    B3MultiSigWallet,
}

impl fmt::Display for ReleaseNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseNames::B3Wallet => write!(f, "b3_wallet"),
            ReleaseNames::B3SimpleWallet => write!(f, "b3_basic_wallet"),
            ReleaseNames::B3MultiSigWallet => write!(f, "b3_multi_sig_wallet"),
            ReleaseNames::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl FromStr for ReleaseNames {
    type Err = HelperError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "b3_wallet" => Ok(ReleaseNames::B3Wallet),
            "b3_basic_wallet" => Ok(ReleaseNames::B3SimpleWallet),
            "b3_multi_sig_wallet" => Ok(ReleaseNames::B3MultiSigWallet),
            name if name.starts_with("custom_") => Ok(ReleaseNames::Custom(name.to_string())),
            _ => Err(HelperError::InvalidReleaseName(name.to_string())),
        }
    }
}

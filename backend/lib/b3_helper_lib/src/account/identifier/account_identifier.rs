use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::subaccount::Subaccount;

use easy_hasher::easy_hasher;
use std::{fmt, str::FromStr};

use super::error::AccountIdentifierError;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AccountIdentifier(pub [u8; 32]);

impl Default for AccountIdentifier {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl AccountIdentifier {
    pub fn new(owner: Principal, subaccount: Subaccount) -> Self {
        let mut data = Vec::new();
        data.push(0x0A);
        data.extend_from_slice("account-id".as_bytes());
        data.extend_from_slice(owner.as_slice());
        data.extend_from_slice(subaccount.as_slice());

        let account_hash = easy_hasher::raw_sha224(data);

        let crc32_hash = easy_hasher::raw_crc32(account_hash.to_vec());

        let mut result = [0u8; 32];
        result[0..4].copy_from_slice(&crc32_hash.to_vec());
        result[4..32].copy_from_slice(&account_hash.to_vec());

        Self(result)
    }
}

impl From<Vec<u8>> for AccountIdentifier {
    fn from(bytes: Vec<u8>) -> Self {
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes[..]);

        Self(result)
    }
}

impl FromStr for AccountIdentifier {
    type Err = AccountIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [0u8; 32];
        let mut i = 0;
        for byte in s.as_bytes().chunks(2) {
            if byte.len() != 2 {
                return Err(AccountIdentifierError::InvalidLength);
            }
            result[i] = u8::from_str_radix(
                std::str::from_utf8(byte)
                    .map_err(|_| AccountIdentifierError::InvalidAccountIdentifier)?,
                16,
            )
            .map_err(|_| AccountIdentifierError::InvalidAccountIdentifier)?;
            i += 1;
        }

        Ok(Self(result))
    }
}

impl fmt::Display for AccountIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

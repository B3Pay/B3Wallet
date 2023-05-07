use std::fmt::Display;

use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
use sha2::Digest;

use crate::error::SignerError;
use crate::ledger::subaccount::Subaccount;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AccountIdentifier(pub [u8; 32]);

impl Default for AccountIdentifier {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl TryFrom<String> for AccountIdentifier {
    type Error = SignerError;

    fn try_from(str: String) -> Result<Self, SignerError> {
        let mut result = [0u8; 32];
        let mut i = 0;
        for byte in str.as_bytes().chunks(2) {
            if byte.len() != 2 {
                return Err(SignerError::InvalidAccountIdentifier);
            }
            result[i] = u8::from_str_radix(
                std::str::from_utf8(byte).map_err(|_| SignerError::InvalidAccountIdentifier)?,
                16,
            )
            .map_err(|_| SignerError::InvalidAccountIdentifier)?;
            i += 1;
        }
        Ok(Self(result))
    }
}

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for byte in self.0.iter() {
            result.push_str(&format!("{:02x}", byte));
        }
        write!(f, "{}", result)
    }
}

impl AccountIdentifier {
    pub fn new(owner: &Principal, subaccount: &Subaccount) -> Self {
        let mut hasher = sha2::Sha224::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(owner.as_slice());
        hasher.update(&subaccount.0[..]);
        let hash: [u8; 28] = hasher.finalize().into();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&hash);
        let crc32_bytes = hasher.finalize().to_be_bytes();

        let mut result = [0u8; 32];
        result[0..4].copy_from_slice(&crc32_bytes[..]);
        result[4..32].copy_from_slice(hash.as_ref());
        Self(result)
    }
}

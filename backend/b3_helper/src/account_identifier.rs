use std::fmt::Display;

use ic_cdk::export::Principal;
use sha2::{Digest, Sha224};

use crate::{
    error::SharedError,
    types::{AccountIdentifier, Subaccount},
};

impl Default for AccountIdentifier {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl AccountIdentifier {
    pub fn new(owner: &Principal, subaccount: &Subaccount) -> Self {
        let mut hasher = Sha224::new();
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

impl TryFrom<String> for AccountIdentifier {
    type Error = SharedError;

    fn try_from(str: String) -> Result<Self, SharedError> {
        let mut result = [0u8; 32];
        let mut i = 0;
        for byte in str.as_bytes().chunks(2) {
            if byte.len() != 2 {
                return Err(SharedError::InvalidAccountIdentifier);
            }
            result[i] = u8::from_str_radix(
                std::str::from_utf8(byte).map_err(|_| SharedError::InvalidAccountIdentifier)?,
                16,
            )
            .map_err(|_| SharedError::InvalidAccountIdentifier)?;
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

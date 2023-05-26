use crate::{
    error::HelperError,
    types::{AccountIdentifier, Environment, Subaccount},
};
use ic_cdk::export::Principal;
use sha3::{Digest, Sha3_224};
use std::{fmt, mem::size_of};

impl Default for AccountIdentifier {
    fn default() -> Self {
        Self([0u8; 32])
    }
}

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0u8; 32])
    }
}

impl Subaccount {
    pub fn new(environment: Environment, nonce: u64) -> Self {
        let mut subaccount = [0; 32];

        match environment {
            Environment::Production => subaccount[0] = 32,
            Environment::Staging => subaccount[0] = 16,
            Environment::Development => subaccount[0] = 8,
        }

        if nonce >= 255 {
            let count = ((nonce - 1) / 255) as usize;
            subaccount[1..=count].fill(255);
            subaccount[count + 1] = (nonce - (count as u64) * 255 - 1) as u8;
        } else {
            subaccount[1] = nonce as u8;
        }

        Subaccount(subaccount)
    }
}

impl From<Principal> for Subaccount {
    fn from(principal: Principal) -> Self {
        let mut subaccount = [0; size_of::<Subaccount>()];
        let principal_id = principal.as_slice();

        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

        Subaccount(subaccount)
    }
}

impl fmt::Display for Subaccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for byte in self.0.iter() {
            result.push_str(&format!("{:02x}", byte));
        }
        write!(f, "{}", result)
    }
}

impl AccountIdentifier {
    pub fn new(owner: Principal, subaccount: Subaccount) -> Self {
        let mut hasher = Sha3_224::new();
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

impl From<Vec<u8>> for AccountIdentifier {
    fn from(bytes: Vec<u8>) -> Self {
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes[..]);
        Self(result)
    }
}

impl TryFrom<String> for AccountIdentifier {
    type Error = HelperError;

    fn try_from(str: String) -> Result<Self, HelperError> {
        let mut result = [0u8; 32];
        let mut i = 0;
        for byte in str.as_bytes().chunks(2) {
            if byte.len() != 2 {
                return Err(HelperError::InvalidAccountIdentifier);
            }
            result[i] = u8::from_str_radix(
                std::str::from_utf8(byte).map_err(|_| HelperError::InvalidAccountIdentifier)?,
                16,
            )
            .map_err(|_| HelperError::InvalidAccountIdentifier)?;
            i += 1;
        }
        Ok(Self(result))
    }
}

impl fmt::Display for AccountIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for byte in self.0.iter() {
            result.push_str(&format!("{:02x}", byte));
        }
        write!(f, "{}", result)
    }
}

use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};

use crate::subaccount::Subaccount;

use easy_hasher::easy_hasher;
use std::{fmt, str::FromStr};

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
        data.extend_from_slice(subaccount.0.as_ref());

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
        let mut result = String::new();
        for byte in self.0.iter() {
            result.push_str(&format!("{:02x}", byte));
        }
        write!(f, "{}", result)
    }
}

pub enum AccountIdentifierError {
    InvalidLength,
    InvalidAccountIdentifier,
}

#[rustfmt::skip]
impl fmt::Display for AccountIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdentifierError::InvalidLength => write!(f, "Invalid length"),
            AccountIdentifierError::InvalidAccountIdentifier => write!(f, "Invalid account identifier")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::environment::Environment;

    use super::*;

    #[test]
    fn test_default_account_identifier() {
        let account_id = AccountIdentifier::default();
        assert_eq!(
            account_id.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000",
        );

        let subaccount = Subaccount::default();

        assert_eq!(
            subaccount.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );

        let account_id = AccountIdentifier::new(Principal::from_slice(&[0, 32]), subaccount);

        assert_eq!(
            account_id.to_string(),
            "ee918f38cb6becc036378e1cb83ad44938ddb5de6e61d243d3351889b5a9536f".to_string()
        );
    }

    #[test]
    fn test_account_identifier() {
        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let subaccount = Subaccount([0; 32]);

        let account_id = AccountIdentifier::new(principal, subaccount);
        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );

        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id =
            AccountIdentifier::new(principal, Subaccount::new(Environment::Production, 0));
        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );

        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id =
            AccountIdentifier::new(principal, Subaccount::new(Environment::Production, 1));

        assert_eq!(
            account_id.to_string(),
            "40900242935be3ae43f9f07262af078486d87f5eb8707da705d2605a6c2f1c9b"
        );
    }
}

use crate::{
    error::HelperError,
    types::{AccountIdentifier, Environment, Subaccount},
};
use easy_hasher::easy_hasher;
use ic_cdk::export::Principal;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subaccount() {
        let env = Environment::Production;
        let subaccount = Subaccount::new(env, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
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
            "9d4f8f6c5ef4767dbe7a933f3e95bb30f3f8e7d6b833c90871e5bbd3213aad87"
        );
    }
}

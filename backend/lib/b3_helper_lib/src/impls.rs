use crate::{
    account::ICRCAccount,
    base32::base32_decode,
    error::{HelperError, TrapError},
    types::{AccountIdentifier, CanisterId, Environment, Subaccount},
};

use easy_hasher::easy_hasher;
use ic_cdk::export::Principal;
use std::{cmp, fmt, hash, mem::size_of, ops::Add, str::FromStr};

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

    pub fn is_default(&self) -> bool {
        self.0 == [0u8; 32]
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, HelperError> {
        if slice.len() != 32 {
            return Err(HelperError::SubaccountSliceError);
        }

        let mut subaccount = [0; 32];
        subaccount.copy_from_slice(slice);

        Ok(Subaccount(subaccount))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn from_hex(hex: &str) -> Result<Self, HelperError> {
        // add leading zeros if necessary
        let hex = if hex.len() < 64 {
            let mut hex = hex.to_string();
            hex.insert_str(0, &"0".repeat(64 - hex.len()));
            hex
        } else {
            hex.to_string()
        };

        let bytes = hex::decode(hex).map_err(|e| HelperError::SubaccountHexError(e.to_string()))?;

        Subaccount::from_slice(&bytes)
    }

    pub fn from_base32(base32: &str) -> Result<Self, HelperError> {
        let bytes =
            base32_decode(base32).map_err(|e| HelperError::SubaccountBase32Error(e.to_string()))?;
        Subaccount::from_slice(&bytes)
    }
}

impl Subaccount {
    pub fn account_identifier(&self, owner: CanisterId) -> AccountIdentifier {
        AccountIdentifier::new(owner, self.clone())
    }

    pub fn icrc1_account(&self, owner: CanisterId) -> ICRCAccount {
        ICRCAccount::new(owner, Some(self.clone()))
    }

    pub fn environment(&self) -> Environment {
        match self.0[0] {
            16 => Environment::Staging,
            8 => Environment::Development,
            _ => Environment::Production,
        }
    }

    pub fn nonce(&self) -> u64 {
        self.0[1..].iter().fold(0, |acc, x| acc + *x as u64)
    }

    pub fn name(&self) -> String {
        self.environment().to_name(self.nonce().add(1).to_string())
    }
}

impl Eq for Subaccount {}

impl cmp::PartialOrd for Subaccount {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Subaccount {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl hash::Hash for Subaccount {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
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

impl From<[u8; 32]> for Subaccount {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl TryFrom<Vec<u8>> for Subaccount {
    type Error = HelperError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(HelperError::InvalidSubaccount(format!(
                "Subaccount must be 32 bytes long, but was {}",
                value.len()
            )));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&value);

        Ok(Self(bytes))
    }
}

impl TryFrom<&str> for Subaccount {
    type Error = HelperError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes =
            hex::decode(value).map_err(|e| HelperError::InvalidSubaccount(e.to_string()))?;

        Ok(Self::try_from(bytes)?)
    }
}

impl fmt::Display for Subaccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        hex::encode(&self.0).fmt(f)
    }
}

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

pub enum AccountIdentifierError {
    InvalidLength,
    InvalidAccountIdentifier,
}

impl TrapError for AccountIdentifierError {
    fn to_string(self) -> String {
        match self {
            AccountIdentifierError::InvalidLength => "Invalid length".to_string(),
            AccountIdentifierError::InvalidAccountIdentifier => {
                "Invalid account identifier".to_string()
            }
        }
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

#[cfg(test)]
mod tests {
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
    fn test_subaccount() {
        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                32, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        assert_eq!(
            subaccount.to_hex(),
            "2001000000000000000000000000000000000000000000000000000000000000"
        );

        let subaccount = Subaccount::try_from(
            "2001000000000000000000000000000000000000000000000000000000000000",
        );

        assert_eq!(
            subaccount.expect("REASON").to_owned(),
            Subaccount([
                32, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
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

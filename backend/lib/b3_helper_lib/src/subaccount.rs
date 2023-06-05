use crate::{
    account::ICRCAccount,
    base32::base32_decode,
    constants::{DEVELOPMENT_PREFIX, STAGING_PREFIX},
    environment::Environment,
    error::SubaccountError,
    identifier::AccountIdentifier,
    types::CanisterId,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};

use std::{cmp, fmt, hash, mem::size_of, ops::Add};

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0u8; 32])
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Subaccount(pub [u8; 32]);

impl Subaccount {
    /// Creates a new `Subaccount` with a given environment and nonce.
    ///
    /// # Arguments
    ///
    /// * `environment` - An `Environment` enum indicating the environment where the subaccount is being created.
    /// * `nonce` - A unique value used to generate different subaccounts.
    ///
    /// # Returns
    ///
    /// * `Subaccount` - A new `Subaccount` struct instance.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_helper_lib::{environment::Environment, subaccount::{ Subaccount}};
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.id(), "account_123456789");
    /// assert_eq!(subaccount.nonce(), 123456789);
    /// assert_eq!(subaccount.environment(), Environment::Production);
    /// ```
    pub fn new(environment: Environment, nonce: u64) -> Self {
        let mut subaccount = [0; 32];
        // Set the 24th byte of the subaccount array as the prefix of the environment
        subaccount[23] = environment.prefix();

        // Convert the nonce into bytes in big-endian order
        let nonce_bytes = nonce.to_be_bytes();
        // Copy the nonce bytes into the subaccount array starting from the 25th byte
        // This leaves the first 24 bytes of the subaccount array as 0 (or the environment prefix at the 24th byte),
        // and the rest of the array as the nonce in big-endian order
        subaccount[24..].copy_from_slice(&nonce_bytes);

        Subaccount(subaccount)
    }

    pub fn environment(&self) -> Environment {
        if self.0[0] == 29 {
            return Environment::Production;
        }
        match self.0[23] {
            STAGING_PREFIX => Environment::Staging,
            DEVELOPMENT_PREFIX => Environment::Development,
            _ => Environment::Production,
        }
    }

    pub fn id(&self) -> String {
        if self.0[0] == 29 {
            return "principal".to_string();
        }

        if self.is_default() {
            return "default".to_string();
        }

        let env_str = match self.0[23] {
            STAGING_PREFIX => "staging_account",
            DEVELOPMENT_PREFIX => "development_account",
            _ => "account",
        };

        let index = self.nonce().to_string();

        [env_str, &index].join("_")
    }

    pub fn name(&self) -> String {
        if self.0[0] == 29 {
            return "Principal".to_string();
        }

        if self.is_default() {
            return "Default".to_string();
        }

        let next_index = self.nonce().add(1).to_string();

        self.environment().to_name(next_index)
    }

    pub fn nonce(&self) -> u64 {
        if self.0[0] == 29 {
            return 0;
        }

        let nonce_bytes = &self.0[24..];
        u64::from_be_bytes(nonce_bytes.try_into().unwrap())
    }

    pub fn is_default(&self) -> bool {
        self.0 == [0u8; 32]
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, SubaccountError> {
        if slice.len() != 32 {
            return Err(SubaccountError::SliceError(
                "Slice must be 32 bytes long".to_string(),
            ));
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

    pub fn from_hex(hex: &str) -> Result<Self, SubaccountError> {
        // add leading zeros if necessary
        let hex = if hex.len() < 64 {
            let mut hex = hex.to_string();
            hex.insert_str(0, &"0".repeat(64 - hex.len()));
            hex
        } else {
            hex.to_string()
        };

        let bytes = hex::decode(hex).map_err(|e| SubaccountError::HexError(e.to_string()))?;

        Subaccount::from_slice(&bytes)
    }

    pub fn from_base32(base32: &str) -> Result<Self, SubaccountError> {
        let bytes =
            base32_decode(base32).map_err(|e| SubaccountError::Base32Error(e.to_string()))?;
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
    type Error = SubaccountError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(SubaccountError::InvalidSubaccount(format!(
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
    type Error = SubaccountError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes =
            hex::decode(value).map_err(|e| SubaccountError::InvalidSubaccount(e.to_string()))?;

        Ok(Self::try_from(bytes)?)
    }
}

impl fmt::Display for Subaccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        hex::encode(&self.0).fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::mocks::ic_cdk_id;

    use super::*;

    #[test]
    fn test_production_subaccount() {
        let subaccount = Subaccount::default();
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 1);

        assert_eq!(subaccount.nonce(), 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(
            subaccount.to_hex(),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );

        let subaccount = Subaccount::try_from(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )
        .expect("Failed to parse subaccount");

        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 512);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0
            ])
        )
    }

    #[test]
    fn test_development_subaccount() {
        let subaccount = Subaccount::new(Environment::Development, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Development, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(
            subaccount.to_hex(),
            "0000000000000000000000000000000000000000000000ff0000000000000001"
        );

        let subaccount = Subaccount::from_hex(
            &"0000000000000000000000000000000000000000000000ff0000000000000001",
        )
        .expect("Failed to parse subaccount");

        assert_eq!(subaccount, Subaccount::new(Environment::Development, 1));
    }

    #[test]
    fn test_staging_subaccount() {
        let subaccount = Subaccount::new(Environment::Staging, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Staging, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(
            subaccount.to_hex(),
            "0000000000000000000000000000000000000000000000aa0000000000000001"
        );
    }

    #[test]
    fn test_account_and_subaccount_with_loop() {
        for i in 0..30 {
            let env = match i % 3 {
                0 => Environment::Production,
                1 => Environment::Staging,
                2 => Environment::Development,
                _ => unreachable!(),
            };
            let nonce = i / 3;

            let subaccount = Subaccount::new(env.clone(), nonce);
            let account = ICRCAccount::new(ic_cdk_id(), Some(subaccount.clone()));

            assert_eq!(account.effective_subaccount(), &subaccount);
            println!("{}", account.to_text());

            let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
            assert_eq!(recover.effective_subaccount().environment(), env);
            assert_eq!(recover.effective_subaccount().nonce(), nonce);

            println!("{:?}", recover);
            assert_eq!(recover, account);
        }
    }
}

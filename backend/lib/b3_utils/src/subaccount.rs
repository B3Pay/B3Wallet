use std::ops::Add;

use super::constants::{DEVELOPMENT_PREFIX_NUMBER, STAGING_PREFIX_NUMBER};
use crate::environment::Environment;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub mod error;
mod test;

pub use error::*;

pub mod traits;

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0u8; 32])
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Subaccount(pub [u8; 32]);

impl Subaccount {
    /// Creates a new `Subaccount` with a given environment and nonce.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.id(), "account_123456789");
    /// assert_eq!(subaccount.nonce(), 123456789);
    /// assert_eq!(subaccount.environment(), Environment::Production);
    /// ```
    pub fn new(environment: Environment, nonce: u64) -> Self {
        let mut subaccount = [0; 32];
        // Set the 24th byte of the subaccount array as the prefix of the environment
        subaccount[23] = environment.identifier();

        // Convert the nonce into bytes in big-endian order
        let nonce_bytes = nonce.to_be_bytes();
        // Copy the nonce bytes into the subaccount array starting from the 25th byte
        // This leaves the first 24 bytes of the subaccount array as 0 (or the environment prefix at the 24th byte),
        // and the rest of the array as the nonce in big-endian order
        // with this we get smallest ICRCAccount ids
        subaccount[24..].copy_from_slice(&nonce_bytes);

        Subaccount(subaccount)
    }

    /// Creates a new `Subaccount` with a given environment and nonce.
    /// This method is used to create subaccounts for the principal.
    /// The nonce is set to 0.
    /// The environment is set to production.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    /// use candid::Principal;
    ///
    /// let principal = Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap();
    ///
    /// let subaccount = Subaccount::from_principal(principal);
    ///
    /// assert_eq!(subaccount.id(), "principal_2chl6-4hpzw-vqaaa-aaaaa-c");
    /// assert_eq!(subaccount.nonce(), 0);
    /// assert_eq!(subaccount.environment(), Environment::Production);
    ///
    /// let principal = Principal::from_text("b7pqa-qqaaa-aaaap-abdva-cai").unwrap();
    ///
    /// let subaccount = Subaccount::from_principal(principal);
    ///
    /// assert_eq!(subaccount.id(), "principal_b7pqa-qqaaa-aaaap-abdva-cai");
    /// assert_eq!(subaccount.nonce(), 0);
    /// assert_eq!(subaccount.environment(), Environment::Production);
    /// ```
    pub fn from_principal(principal: Principal) -> Self {
        principal.into()
    }

    /// Creates a new `Subaccount` with a given environment and nonce.
    /// This method is used to create subaccounts for the principal.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    /// use candid::Principal;
    ///
    /// let principal = Principal::from_text("b7pqa-qqaaa-aaaap-abdva-cai").unwrap();
    ///
    /// let subaccount = Subaccount::from_principal(principal);
    /// assert_eq!(subaccount.environment(), Environment::Production);
    ///
    /// let subaccount = Subaccount::new(Environment::Development, 123456789);
    /// assert_eq!(subaccount.environment(), Environment::Development);
    /// ```
    pub fn environment(&self) -> Environment {
        if self.is_principal() {
            return Environment::Production;
        }

        match self.0[23] {
            STAGING_PREFIX_NUMBER => Environment::Staging,
            DEVELOPMENT_PREFIX_NUMBER => Environment::Development,
            _ => Environment::Production,
        }
    }

    /// Returns the id of the subaccount.
    /// The id is used to identify the subaccount in the backend.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    /// use candid::Principal;
    ///
    /// let principal = Principal::from_text("b7pqa-qqaaa-aaaap-abdva-cai").unwrap();
    ///
    /// let subaccount = Subaccount::from_principal(principal);
    /// assert_eq!(subaccount.id(), "principal_b7pqa-qqaaa-aaaap-abdva-cai");
    ///
    /// let subaccount = Subaccount::new(Environment::Development, 123456789);
    /// assert_eq!(subaccount.id(), "development_account_123456789");
    ///
    /// let subaccount = Subaccount::new(Environment::Staging, 123456789);
    /// assert_eq!(subaccount.id(), "staging_account_123456789");
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.id(), "account_123456789");
    /// ```
    pub fn id(&self) -> String {
        if let Ok(principal) = self.to_principal() {
            return format!("principal_{}", principal);
        }

        if self.is_default() {
            return "-default".to_string();
        }

        let env_str = match self.0[23] {
            STAGING_PREFIX_NUMBER => "staging_account",
            DEVELOPMENT_PREFIX_NUMBER => "development_account",
            _ => "account",
        };

        let index = self.nonce().to_string();

        [env_str, &index].join("_")
    }

    /// returns the account name of the subaccount
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.name(), "Account 123456790");
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 0);
    /// assert_eq!(subaccount.name(), "Default");
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 1);
    /// assert_eq!(subaccount.name(), "Account 2");
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 2);
    /// assert_eq!(subaccount.name(), "Account 3");
    /// ```
    pub fn name(&self) -> String {
        if self.is_principal() {
            return "Principal".to_string();
        }

        if self.is_default() {
            return "Default".to_string();
        }

        let next_index = self.nonce().add(1);

        self.environment().to_name(next_index)
    }

    /// returns the nonce of the subaccount
    /// The nonce is the last 24 bytes of the subaccount
    /// if first byte of the subaccount id is 0 then its an Account
    /// otherwise its an Principal
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    ///
    /// let subaccount = Subaccount::from_principal("2chl6-4hpzw-vqaaa-aaaaa-c".parse().unwrap());
    /// assert_eq!(subaccount.nonce(), 0);
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.nonce(), 123456789);
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 1);
    /// assert_eq!(subaccount.nonce(), 1);
    /// ```
    pub fn nonce(&self) -> u64 {
        if self.is_principal() {
            return 0;
        }

        let nonce_bytes = &self.0[24..];
        u64::from_be_bytes(nonce_bytes.try_into().unwrap())
    }

    /// Checks if the subaccount is the default subaccount
    /// The default subaccount is the first Production subaccount of an account
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    ///
    /// let subaccount = Subaccount::from_principal("2chl6-4hpzw-vqaaa-aaaaa-c".parse().unwrap());
    /// assert_eq!(subaccount.is_default(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.is_default(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Development, 0);
    /// assert_eq!(subaccount.is_default(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Staging, 0);
    /// assert_eq!(subaccount.is_default(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 0);
    /// assert_eq!(subaccount.is_default(), true);
    /// ```
    pub fn is_default(&self) -> bool {
        self.0 == [0u8; 32]
    }

    /// Checks if the subaccount is a principal subaccount
    /// A principal subaccount is a subaccount that is not the default subaccount
    /// and has a principal id
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    ///
    /// let subaccount = Subaccount::from_principal("2chl6-4hpzw-vqaaa-aaaaa-c".parse().unwrap());
    /// assert_eq!(subaccount.is_principal(), true);
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 123456789);
    /// assert_eq!(subaccount.is_principal(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Development, 0);
    /// assert_eq!(subaccount.is_principal(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Staging, 0);
    /// assert_eq!(subaccount.is_principal(), false);
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 0);
    /// assert_eq!(subaccount.is_principal(), false);
    /// ```
    pub fn is_principal(&self) -> bool {
        self.0[0] != 0
    }

    /// Returns the subaccount from slice.
    /// Error if the slice is not 32 bytes long.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::Subaccount;
    ///
    /// let subaccount = Subaccount::from_slice(&[0u8; 32]).unwrap();
    /// assert!(subaccount.is_default());
    ///
    /// let subaccount = Subaccount::from_slice(&[1u8; 32]).unwrap();
    /// assert_eq!(subaccount.to_string(), "0101010101010101010101010101010101010101010101010101010101010101".to_string());
    ///
    /// let subaccount = Subaccount::from_slice(&[2u8; 32]).unwrap();
    /// assert_eq!(subaccount.to_string(), "0202020202020202020202020202020202020202020202020202020202020202".to_string());
    ///
    /// let subaccount = Subaccount::from_slice(&[0u8; 33]);
    /// assert!(subaccount.is_err());
    /// ```
    pub fn from_slice(slice: &[u8]) -> Result<Self, SubaccountError> {
        if slice.len() != 32 {
            return Err(SubaccountError::LengthError(slice.len()));
        }

        let mut subaccount = [0; 32];
        subaccount.copy_from_slice(slice);

        Ok(Subaccount(subaccount))
    }

    pub fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Returns the subaccount as a Principal.
    /// Panics if the slice is longer than 29 bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    /// use candid::Principal;
    ///
    /// let principal = Principal::from_text("b7pqa-qqaaa-aaaap-abdva-cai").unwrap();
    ///
    /// let subaccount = Subaccount::from_principal(principal);
    ///
    /// assert_eq!(subaccount.to_principal().unwrap().to_text(), "b7pqa-qqaaa-aaaap-abdva-cai");
    /// ```
    pub fn to_principal(&self) -> Result<Principal, SubaccountError> {
        if !self.is_principal() {
            return Err(SubaccountError::NotPrincipal);
        }

        let length = self.0[0] as usize;

        if length > 29 {
            return Err(SubaccountError::LengthError(length));
        }

        let principal_slice = self.0[1..length + 1].to_vec();

        let principal = Principal::from_slice(&principal_slice);

        Ok(principal)
    }

    /// Returns the hex representation of the subaccount
    /// with leading zeros removed
    /// e.g. 0000000
    /// will be returned as 0
    /// 0000001
    /// will be returned as 1
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    /// use candid::Principal;
    ///
    /// let subaccount = Subaccount::new(Environment::Production, 0);
    /// assert_eq!(subaccount.to_hex(), "".to_string());
    ///
    /// let subaccount = Subaccount::from_principal("2chl6-4hpzw-vqaaa-aaaaa-c".parse().unwrap());
    /// assert_eq!(subaccount.to_hex(), "9efcdab00000000000100000000000000000000000000000000000000000000".to_string());
    /// ```
    pub fn to_hex(&self) -> String {
        hex::encode(&self.as_slice())
            .trim_start_matches('0')
            .to_owned()
    }

    /// Returns the hex representation of the subaccount
    /// with add leading zeros if necessary
    ///
    /// # Example
    ///
    /// ```
    /// use b3_utils::{Environment, Subaccount};
    /// use candid::Principal;
    ///
    /// let subaccount = Subaccount::from_hex("").unwrap();
    /// assert!(subaccount.is_default());
    ///
    /// let subaccount = Subaccount::from_hex("test");
    /// assert!(subaccount.is_err());
    ///
    /// let subaccount = Subaccount::from_hex("1").unwrap();
    /// assert_eq!(subaccount.id(), "account_1");
    ///
    /// let subaccount = Subaccount::from_hex("ff00000000000004d2").unwrap();
    /// assert_eq!(subaccount.id(), "development_account_1234");
    /// assert_eq!(subaccount.nonce(), 1234);
    /// assert_eq!(subaccount.environment(), Environment::Development);
    /// assert_eq!(subaccount.id(), "development_account_1234");
    ///
    /// let subaccount = Subaccount::from_hex("aa00000000075bcd15").unwrap();
    /// assert_eq!(subaccount.id(), "staging_account_123456789");
    /// assert_eq!(subaccount.nonce(), 123456789);
    /// assert_eq!(subaccount.environment(), Environment::Staging);
    ///
    /// let subaccount = Subaccount::from_hex("9efcdab00000000000100000000000000000000000000000000000000000000").unwrap();
    /// assert!(subaccount.is_principal());
    /// assert_eq!(subaccount.to_principal().unwrap().to_text(), "2chl6-4hpzw-vqaaa-aaaaa-c");
    /// ```
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

    pub fn derivation_id(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn derivation_path(&self) -> Vec<Vec<u8>> {
        vec![self.0.to_vec()]
    }
}

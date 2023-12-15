use b3_utils::{
    api::Management,
    ledger::{constants::SYSTEM_RATE_LIMIT, Metadata},
    memory::types::{Bound, Storable},
    types::{CanisterId, CanisterIds, ControllerId},
    NanoTimeStamp,
};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ic_cdk::api::management_canister::{
    main::CreateCanisterArgument, provisional::CanisterSettings,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use super::{error::UserSystemError, types::UserView};

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    canisters: Vec<CanisterId>,
    created_at: NanoTimeStamp,
    updated_at: NanoTimeStamp,
    metadata: Metadata,
}

// Create the User struct
impl User {
    pub fn new(opt_canister_id: Option<CanisterId>) -> Self {
        let mut canisters = Vec::new();

        if let Some(canister_id) = opt_canister_id {
            canisters.push(canister_id);
        }

        Self {
            canisters,
            updated_at: NanoTimeStamp::now(),
            created_at: NanoTimeStamp::now(),
            metadata: Metadata::new(),
        }
    }
}

// Write to the User struct
impl User {
    /// get with updated_at.
    pub fn update_rate(&mut self) -> Result<User, UserSystemError> {
        self.check_rate()?;
        self.updated_at = NanoTimeStamp::now();

        Ok(self.clone())
    }

    /// add the canister id.
    pub fn add_canister(&mut self, canister_id: CanisterId) {
        self.canisters.push(canister_id);
        self.updated_at = NanoTimeStamp::now();
    }

    /// remove the canister id.
    pub fn remove_canister(&mut self, canister_id: CanisterId) -> Result<(), UserSystemError> {
        let index = self
            .canisters
            .iter()
            .position(|id| id == &canister_id)
            .ok_or(UserSystemError::WalletCanisterNotFound)?;

        self.canisters.remove(index);
        self.updated_at = NanoTimeStamp::now();

        Ok(())
    }

    /// create a new canister and save the canister id.
    pub async fn create_with_cycles(
        &mut self,
        controllers: Vec<ControllerId>,
        cycles: u128,
    ) -> Result<CanisterId, UserSystemError> {
        let args = CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(controllers.clone()),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            }),
        };

        let result = Management::create_canister(args, cycles).await;

        match result {
            Ok(result) => {
                let canister_id = result.canister_id;

                self.add_canister(canister_id);

                Ok(canister_id)
            }
            Err(err) => Err(UserSystemError::CreateCanisterError(err.to_string())),
        }
    }
}

// Read from the User struct
impl User {
    pub fn view(&self) -> UserView {
        UserView {
            canisters: self.canisters.clone(),
            updated_at: self.updated_at.clone(),
            created_at: self.created_at.clone(),
            metadata: self.metadata.clone(),
        }
    }

    /// Returns the canister ids, throws an error if it is not available.
    pub fn canisters(&self) -> CanisterIds {
        self.canisters.clone()
    }

    /// Make an function that use updated_at and check the rate of the user.
    pub fn check_rate(&self) -> Result<(), UserSystemError> {
        if self.updated_at.rate_limit_exceeded(SYSTEM_RATE_LIMIT) {
            return Err(UserSystemError::RateLimitExceeded);
        } else {
            Ok(())
        }
    }
}

impl From<CanisterId> for User {
    fn from(canister_id: CanisterId) -> Self {
        let mut canisters = Vec::new();

        canisters.push(canister_id);

        Self {
            canisters,
            metadata: Metadata::new(),
            updated_at: NanoTimeStamp::now(),
            created_at: NanoTimeStamp::now(),
        }
    }
}

impl Storable for User {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

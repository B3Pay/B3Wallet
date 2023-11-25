use crate::{error::SystemError, types::Canisters, wallet::WalletCanister};
use b3_utils::{
    ledger::constants::SYSTEM_RATE_LIMIT,
    memory::types::{Bound, Storable},
    types::{CanisterId, ControllerId},
    NanoTimeStamp,
};
use candid::CandidType;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ic_cdk::api::management_canister::{
    main::{create_canister, CreateCanisterArgument},
    provisional::CanisterSettings,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct UserState {
    pub canisters: Vec<WalletCanister>,
    pub created_at: NanoTimeStamp,
    pub updated_at: NanoTimeStamp,
}

impl From<WalletCanister> for UserState {
    fn from(canister_id: WalletCanister) -> Self {
        Self {
            canisters: vec![canister_id],
            updated_at: NanoTimeStamp::now(),
            created_at: NanoTimeStamp::now(),
        }
    }
}

impl Storable for UserState {
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

impl UserState {
    /// Create a new canister.
    pub fn new(opt_canister_id: Option<CanisterId>) -> Self {
        Self {
            canisters: opt_canister_id
                .map_or(vec![], |canister_id| vec![WalletCanister::new(canister_id)]),
            updated_at: NanoTimeStamp::now(),
            created_at: NanoTimeStamp::now(),
        }
    }

    /// get with updated_at.
    pub fn update_rate(&mut self) -> Result<UserState, SystemError> {
        self.check_rate()?;
        self.updated_at = NanoTimeStamp::now();

        Ok(self.clone())
    }

    /// Set the canister id.
    pub fn add_canister(&mut self, canister_id: WalletCanister) {
        if self.canisters.contains(&canister_id) {
            return;
        }
        self.canisters.push(canister_id);
        self.updated_at = NanoTimeStamp::now();
    }

    /// Change the canister id.
    pub fn change_canister(&mut self, index: usize, canister_id: CanisterId) {
        self.canisters[index] = canister_id.into();
        self.updated_at = NanoTimeStamp::now();
    }

    /// Returns the canister ids, throws an error if it is not available.
    pub fn canisters(&self) -> Result<Canisters, SystemError> {
        if self.canisters.is_empty() {
            return Err(SystemError::CanisterIdNotFound);
        }

        Ok(self.canisters.clone())
    }

    /// Make an function that use updated_at and check the rate of the user.
    pub fn check_rate(&self) -> Result<(), SystemError> {
        if self.updated_at.rate_limit_exceeded(SYSTEM_RATE_LIMIT) {
            return Err(SystemError::RateLimitExceeded);
        } else {
            Ok(())
        }
    }

    /// create a new canister and save the canister id.
    pub async fn create_with_cycles(
        &mut self,
        controllers: Vec<ControllerId>,
        cycles: u128,
    ) -> Result<WalletCanister, SystemError> {
        let args = CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(controllers.clone()),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            }),
        };

        let result = create_canister(args, cycles).await;

        match result {
            Ok(result) => {
                let canister_id = WalletCanister::new(result.0.canister_id);

                self.add_canister(canister_id.clone());

                Ok(canister_id)
            }
            Err(err) => Err(SystemError::CreateCanisterError(err.1)),
        }
    }
}

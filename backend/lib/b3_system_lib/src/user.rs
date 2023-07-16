use crate::{error::SystemError, types::Canisters, wallet::WalletCanister};
use b3_helper_lib::{
    constants::RATE_LIMIT,
    timestamp::NanoTimeStamp,
    types::{CanisterId, ControllerId},
};
use ic_cdk::{
    api::management_canister::{
        main::{create_canister_with_extra_cycles, CreateCanisterArgument},
        provisional::CanisterSettings,
    },
    export::candid::{CandidType, Deserialize},
};

#[derive(CandidType, Deserialize, Clone)]
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

impl UserState {
    /// Create a new canister.
    pub fn new(opt_canister_id: Option<CanisterId>) -> Self {
        Self {
            canisters: opt_canister_id
                .map_or(vec![], |canister_id| vec![WalletCanister(canister_id)]),
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
        if self.updated_at.rate_limit_exceeded(RATE_LIMIT) {
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

        let result = create_canister_with_extra_cycles(args, cycles).await;

        match result {
            Ok(result) => {
                let canister_id = WalletCanister(result.0.canister_id);

                self.add_canister(canister_id.clone());

                Ok(canister_id)
            }
            Err(err) => Err(SystemError::CreateCanisterError(err.1)),
        }
    }
}

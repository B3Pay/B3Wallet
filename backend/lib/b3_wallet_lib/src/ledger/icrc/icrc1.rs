use b3_utils::{types::CanisterId, ICRCAccount};
use ic_cdk::api::call::call;

use super::{
    error::IcrcError,
    types::{ICRC1TransferArgs, ICRC1TransferResult, ICRCMetadata, ICRCTokens},
};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct ICRC1(pub CanisterId);

impl ICRC1 {
    pub fn new(canister_id: CanisterId) -> Self {
        ICRC1(canister_id)
    }

    pub async fn name(&self) -> Result<String, IcrcError> {
        let (res,): (String,) = call(self.0, "icrc1_name", ())
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn fee(&self) -> Result<ICRCTokens, IcrcError> {
        let (res,): (ICRCTokens,) = call(self.0, "icrc1_fee", ())
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn symbol(&self) -> Result<String, IcrcError> {
        let (res,): (String,) = call(self.0, "icrc1_symbol", ())
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn decimals(&self) -> Result<u8, IcrcError> {
        let (res,): (u8,) = call(self.0, "icrc1_decimals", ())
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn metadata(&self) -> Result<ICRCMetadata, IcrcError> {
        let (res,): (ICRCMetadata,) = call(self.0, "icrc1_metadata", ())
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn total_supply(&self) -> Result<ICRCTokens, IcrcError> {
        let (res,): (ICRCTokens,) = call(self.0, "icrc1_total_supply", ())
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn balance_of(&self, account: ICRCAccount) -> Result<ICRCTokens, IcrcError> {
        let (res,): (ICRCTokens,) = call(self.0, "icrc1_balance_of", (account,))
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }

    pub async fn transfer(
        &self,
        args: ICRC1TransferArgs,
    ) -> Result<ICRC1TransferResult, IcrcError> {
        let (res,): (ICRC1TransferResult,) = call(self.0, "icrc1_transfer", (args,))
            .await
            .map_err(|e| IcrcError::CallError(e.1))?;

        Ok(res)
    }
}

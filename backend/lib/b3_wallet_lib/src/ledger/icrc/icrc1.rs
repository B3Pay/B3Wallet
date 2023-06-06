use b3_helper_lib::types::CanisterId;
use ic_cdk::api::call::call;

use super::types::{ICRCFee, ICRCMetadata};
use crate::error::WalletError;

pub struct ICRC1(pub CanisterId);

impl ICRC1 {
    pub fn new(canister_id: CanisterId) -> Self {
        ICRC1(canister_id)
    }

    pub async fn fee(&self) -> Result<ICRCFee, WalletError> {
        let (res,): (ICRCFee,) = call(self.0, "icrc1_fee", ())
            .await
            .map_err(|e| WalletError::SignError(e.1))?;

        Ok(res)
    }

    pub async fn symbol(&self) -> Result<String, WalletError> {
        let (res,): (String,) = call(self.0, "icrc1_symbol", ())
            .await
            .map_err(|e| WalletError::SignError(e.1))?;

        Ok(res)
    }

    pub async fn metadata(&self) -> Result<ICRCMetadata, WalletError> {
        let (res,): (ICRCMetadata,) = call(self.0, "icrc1_metadata", ())
            .await
            .map_err(|e| WalletError::SignError(e.1))?;

        Ok(res)
    }
}

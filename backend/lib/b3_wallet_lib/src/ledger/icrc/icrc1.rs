use b3_helper_lib::types::CanisterId;
use ic_cdk::api::call::call;

use crate::{error::WalletError, ledger::types::ICRCFee};

pub struct Icrc1(pub CanisterId);

impl Icrc1 {
    pub fn new(canister_id: CanisterId) -> Self {
        Icrc1(canister_id)
    }

    pub async fn fee(&self) -> Result<ICRCFee, WalletError> {
        let (res,): (ICRCFee,) = call(self.0, "icrc1_fee", ())
            .await
            .map_err(|e| WalletError::SignError(e.1))?;

        Ok(res)
    }

    pub async fn metadata(&self) -> Result<Vec<u8>, WalletError> {
        let (res,): (Vec<u8>,) = call(self.0, "icrc1_metadata", ())
            .await
            .map_err(|e| WalletError::SignError(e.1))?;

        Ok(res)
    }
}

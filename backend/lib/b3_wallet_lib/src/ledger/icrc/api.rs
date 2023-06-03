use async_trait::async_trait;
use ic_cdk::api::call::call;

use crate::{
    error::WalletError,
    ledger::types::{Balance, ChainTrait, ICRC},
};

#[async_trait]
impl ChainTrait for ICRC {
    async fn balance(&self) -> Result<Balance, WalletError> {
        let (res,): (Balance,) = call(self.canister_id, "icrc1_balance_of", ())
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res)
    }
}

use crate::error::RequestError;
use crate::request::success::CanisterTopUped;
use crate::request::success::IcpTransfered;
use crate::request::ExecutionResult;
use crate::request::RequestTrait;
use async_trait::async_trait;
use b3_helper_lib::identifier::AccountIdentifier;
use b3_helper_lib::tokens::Tokens;
use b3_helper_lib::types::{CanisterId, Memo, NotifyTopUpResult, TransferResult};
use b3_wallet_lib::error::WalletError;
use b3_wallet_lib::store::with_ledger;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

// TRANSFER ICP
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct IcpTransfer {
    account_id: String,
    to: AccountIdentifier,
    amount: Tokens,
    fee: Option<Tokens>,
    memo: Option<Memo>,
}

#[async_trait]
impl RequestTrait for IcpTransfer {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let result = ledger
            .transfer(
                self.to.clone(),
                self.amount.clone(),
                self.fee.clone(),
                self.memo.clone(),
            )
            .await?;

        match result {
            TransferResult::Ok(block_number) => Ok(IcpTransfered(block_number).into()),
            TransferResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        if self.amount.is_zero() {
            return Err(RequestError::AmountIsZero);
        }

        if self.fee.is_some() && self.fee.as_ref().unwrap().is_zero() {
            return Err(RequestError::FeeIsZero);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "icp_transfer".to_string()
    }
}

// TOP UP CANISTER
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct TopUpCanister {
    account_id: String,
    canister_id: Option<CanisterId>,
    amount: Tokens,
    fee: Option<Tokens>,
}

#[async_trait]
impl RequestTrait for TopUpCanister {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let canister_id = self.canister_id.unwrap_or(ic_cdk_id());

        let result = ledger
            .topup_and_notify_top_up(canister_id, self.amount.clone(), self.fee.clone())
            .await?;

        match result {
            NotifyTopUpResult::Ok(amount) => Ok(CanisterTopUped(amount).into()),
            NotifyTopUpResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        if self.amount.is_zero() {
            return Err(RequestError::AmountIsZero);
        }

        if self.fee.is_some() && self.fee.as_ref().unwrap().is_zero() {
            return Err(RequestError::FeeIsZero);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "top_up_canister".to_string()
    }
}

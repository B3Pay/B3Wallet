use crate::error::PermitError;
use crate::request::request::RequestTrait;
use crate::request::result::CanisterTopUped;
use crate::request::result::ExecutionResult;
use crate::request::result::IcpTransfered;
use crate::request::result::TopUpTransfered;
use async_trait::async_trait;
use b3_helper_lib::icp_token::ICPToken;
use b3_helper_lib::identifier::AccountIdentifier;
use b3_helper_lib::types::{CanisterId, Memo, NotifyTopUpResult, TransferResult};
use b3_wallet_lib::error::WalletError;
use b3_wallet_lib::ledger::types::ChainEnum;
use b3_wallet_lib::store::with_chain;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// TRANSFER ICP
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct IcpTransfer {
    pub account_id: String,
    pub to: AccountIdentifier,
    pub amount: ICPToken,
    pub fee: Option<ICPToken>,
    pub memo: Option<Memo>,
}

#[async_trait]
impl RequestTrait for IcpTransfer {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let icp = with_chain(&self.account_id, &ChainEnum::ICP, |chain| chain.icp())??;

        let result = icp
            .transfer(
                self.to.clone(),
                self.amount.clone(),
                self.fee.clone(),
                self.memo.clone(),
            )
            .await?;

        match result {
            TransferResult::Ok(block_number) => Ok(IcpTransfered(self, block_number).into()),
            TransferResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        if self.amount.is_zero() {
            return Err(PermitError::AmountIsZero);
        }

        if self.fee.is_some() && self.fee.as_ref().unwrap().is_zero() {
            return Err(PermitError::FeeIsZero);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "icp_transfer".to_string()
    }

    fn title(&self) -> String {
        format!("Transfer {} ICP", self.amount)
    }

    fn message(&self) -> String {
        format!("Transfer {} ICP", self.amount)
    }
}

// TOP UP CANISTER
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct TopUpTransfer {
    pub account_id: String,
    pub canister_id: CanisterId,
    pub amount: ICPToken,
    pub fee: Option<ICPToken>,
}

#[async_trait]
impl RequestTrait for TopUpTransfer {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let icp = with_chain(&self.account_id, &ChainEnum::ICP, |chain| chain.icp())??;

        let block_index = icp.top_up(self.canister_id, self.amount.clone()).await?;

        Ok(TopUpTransfered(self, block_index).into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        if self.amount.is_zero() {
            return Err(PermitError::AmountIsZero);
        }

        if self.fee.is_some() && self.fee.as_ref().unwrap().is_zero() {
            return Err(PermitError::FeeIsZero);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "top_up_canister".to_string()
    }

    fn title(&self) -> String {
        format!("Top up {} ICP", self.amount)
    }

    fn message(&self) -> String {
        format!("Top up {} ICP", self.amount)
    }
}

// TOP UP CANISTER
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct NotifyTopUp {
    pub account_id: String,
    pub canister_id: CanisterId,
    pub block_index: u64,
}

#[async_trait]
impl RequestTrait for NotifyTopUp {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let icp = with_chain(&self.account_id, &ChainEnum::ICP, |chain| chain.icp())??;

        let result = icp
            .notify_top_up(self.canister_id, self.block_index)
            .await?;

        match result {
            NotifyTopUpResult::Ok(cycles) => Ok(CanisterTopUped(self, cycles).into()),
            NotifyTopUpResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        Ok(())
    }

    fn method_name(&self) -> String {
        "top_up_canister".to_string()
    }

    fn title(&self) -> String {
        format!("Top up canister {}", self.canister_id)
    }

    fn message(&self) -> String {
        format!("Top up canister {}", self.canister_id)
    }
}

use crate::error::OperationError;
use crate::operation::result::CanisterTopUped;
use crate::operation::result::IcpTransfered;
use crate::operation::result::OperationResult;
use crate::operation::result::TopUpTransfered;
use crate::operation::OperationTrait;
use async_trait::async_trait;
use b3_utils::ledger::currency::ICPToken;
use b3_utils::ledger::types::ICPTransferResult;
use b3_utils::ledger::types::NotifyTopUpResult;
use b3_utils::ledger::types::TransferMemo;
use b3_utils::ledger::AccountIdentifier;
use b3_utils::types::CanisterId;
use b3_wallet_lib::error::WalletError;
use b3_wallet_lib::ledger::types::ChainEnum;
use b3_wallet_lib::store::with_chain;
use candid::{CandidType, Deserialize};

// TRANSFER ICP
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct IcpTransfer {
    pub account_id: String,
    pub to: AccountIdentifier,
    pub amount: ICPToken,
    pub fee: Option<ICPToken>,
    pub memo: Option<TransferMemo>,
}

#[async_trait]
impl OperationTrait for IcpTransfer {
    async fn execute(self) -> Result<OperationResult, WalletError> {
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
            ICPTransferResult::Ok(block_number) => Ok(IcpTransfered(self, block_number).into()),
            ICPTransferResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.amount.is_zero() {
            return Err(OperationError::AmountIsZero);
        }

        if self.fee.is_some() && self.fee.as_ref().unwrap().is_zero() {
            return Err(OperationError::FeeIsZero);
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
impl OperationTrait for TopUpTransfer {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let icp = with_chain(&self.account_id, &ChainEnum::ICP, |chain| chain.icp())??;

        let block_index = icp.top_up(self.canister_id, self.amount.clone()).await?;

        Ok(TopUpTransfered(self, block_index).into())
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.amount.is_zero() {
            return Err(OperationError::AmountIsZero);
        }

        if self.fee.is_some() && self.fee.as_ref().unwrap().is_zero() {
            return Err(OperationError::FeeIsZero);
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
impl OperationTrait for NotifyTopUp {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let icp = with_chain(&self.account_id, &ChainEnum::ICP, |chain| chain.icp())??;

        let result = icp
            .notify_top_up(self.canister_id, self.block_index)
            .await?;

        match result {
            NotifyTopUpResult::Ok(cycles) => Ok(CanisterTopUped(self, cycles).into()),
            NotifyTopUpResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), OperationError> {
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

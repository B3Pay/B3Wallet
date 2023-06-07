use std::fmt;

use super::Request;
use crate::types::ConsentMessageResponse;
use b3_helper_lib::error::ErrorTrait;
use b3_helper_lib::identifier::AccountIdentifier;
use b3_helper_lib::tokens::Tokens;
use b3_helper_lib::types::{CanisterId, Memo, NotifyTopUpResult, TransferResult};
use b3_wallet_lib::error::WalletError;
use b3_wallet_lib::store::with_ledger;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum IcpRequest {
    IcpTransferRequest,
    TopUpCanisterRequest,
}

impl IcpRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        match self {
            IcpRequest::IcpTransferRequest(args) => args.execute().await,
            IcpRequest::TopUpCanisterRequest(args) => args.execute().await,
        }
    }
}

impl fmt::Display for IcpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IcpRequest::IcpTransferRequest(_) => write!(f, "IcpTransferRequest"),
            IcpRequest::TopUpCanisterRequest(_) => write!(f, "TopUpCanisterRequest"),
        }
    }
}

// TRANSFER ICP
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct IcpTransferRequest {
    account_id: String,
    to: AccountIdentifier,
    amount: Tokens,
    fee: Option<Tokens>,
    memo: Option<Memo>,
}

impl From<IcpTransferRequest> for Request {
    fn from(args: IcpTransferRequest) -> Self {
        IcpRequest::IcpTransferRequest(args).into()
    }
}

impl IcpTransferRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
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
            TransferResult::Ok(block_number) => Ok(block_number.to_string().into()),
            TransferResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }
}

// TOP UP CANISTER
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct TopUpCanisterRequest {
    account_id: String,
    canister_id: Option<CanisterId>,
    amount: Tokens,
    fee: Option<Tokens>,
}

impl From<TopUpCanisterRequest> for Request {
    fn from(args: TopUpCanisterRequest) -> Self {
        IcpRequest::TopUpCanisterRequest(args).into()
    }
}

impl TopUpCanisterRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let canister_id = self.canister_id.unwrap_or(ic_cdk_id());

        let result = ledger
            .topup_and_notify_top_up(canister_id, self.amount.clone(), self.fee.clone())
            .await?;

        match result {
            NotifyTopUpResult::Ok(amount) => Ok(amount.to_string().into()),
            NotifyTopUpResult::Err(err) => Err(WalletError::NotifyTopUpError(err.to_string())),
        }
    }
}

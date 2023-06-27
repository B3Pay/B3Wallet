use crate::request::result::ExecutionResult;
use async_trait::async_trait;
use b3_wallet_lib::{
    error::WalletError,
    ledger::{chain::ChainTrait, types::ChainEnum},
    store::with_chain,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::error::PermitError;

use super::{request::RequestTrait, result::TokenSent};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct SendToken {
    pub to: String,
    pub amount: u64,
    pub chain: ChainEnum,
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for SendToken {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let chain = with_chain(&self.account_id, &self.chain, |chain| chain.clone())?;

        let result = chain.send(self.to.clone(), self.amount).await;

        match result {
            Ok(result) => Ok(TokenSent(self, result).into()),
            Err(err) => return Err(WalletError::ExecutionError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        if self.amount == 0 {
            return Err(PermitError::InvalidAmount);
        }

        with_chain(&self.account_id, &self.chain, |_| Ok(()))?
    }

    fn method_name(&self) -> String {
        "send_token".to_string()
    }

    fn title(&self) -> String {
        format!("Send {} {}", self.amount, self.chain)
    }
}

use crate::request::result::ExecutionResult;
use async_trait::async_trait;
use b3_helper_lib::amount::Amount;
use b3_wallet_lib::{
    error::WalletError,
    ledger::{chain::ChainTrait, types::ChainEnum},
    store::{with_account, with_chain},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::error::PermitError;

use super::{request::RequestTrait, result::TokenSent};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct SendToken {
    pub to: String,
    pub amount: Amount,
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
        if self.amount <= Amount::from(0) {
            return Err(PermitError::InvalidAmount);
        }

        with_account(&self.account_id, |account| {
            if account.is_hidden() {
                return Err(PermitError::AccountIsHidden);
            }

            account
                .ledger()
                .chain(&self.chain)
                .map(|_| ())
                .map_err(|_| {
                    PermitError::ChainNotFound(self.chain.to_string(), self.account_id.clone())
                })?;

            Ok(())
        })
        .map_err(|err| PermitError::WalletError(err))?
    }

    fn method_name(&self) -> String {
        "send_token".to_string()
    }

    fn title(&self) -> String {
        format!("Send {} {}", self.amount, self.chain)
    }

    fn message(&self) -> String {
        // we already checked that the account exists on validate_request
        let account = with_account(&self.account_id, |account| account.clone()).unwrap();

        format!(
            "Send {} {} from {}({}) to {}",
            self.amount,
            self.chain,
            account.name(),
            self.account_id,
            self.to
        )
    }
}

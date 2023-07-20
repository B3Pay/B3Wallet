use crate::operation::result::OperationResult;
use async_trait::async_trait;
use b3_utils::currency::TokenAmount;
use b3_wallet_lib::{
    error::WalletError,
    ledger::{chain::ChainTrait, types::ChainEnum},
    store::{with_account, with_chain},
};
use candid::{CandidType, Deserialize};

use crate::error::OperationError;

use super::{operations::OperationTrait, result::TokenSent};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct SendToken {
    pub to: String,
    pub chain: ChainEnum,
    pub amount: TokenAmount,
    pub account_id: String,
}

#[async_trait]
impl OperationTrait for SendToken {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let chain = with_chain(&self.account_id, &self.chain, |chain| chain.clone())?;

        let result = chain.send(self.to.clone(), self.amount).await;

        match result {
            Ok(result) => Ok(TokenSent(self, result).into()),
            Err(err) => return Err(WalletError::ExecutionError(err.to_string())),
        }
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.amount <= TokenAmount::from(0) {
            return Err(OperationError::InvalidAmount);
        }

        with_account(&self.account_id, |account| {
            if account.is_hidden() {
                return Err(OperationError::AccountIsHidden);
            }

            account
                .ledger()
                .chain(&self.chain)
                .map(|_| ())
                .map_err(|_| {
                    OperationError::ChainNotFound(self.chain.to_string(), self.account_id.clone())
                })?;

            Ok(())
        })
        .map_err(|err| OperationError::WalletError(err))?
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

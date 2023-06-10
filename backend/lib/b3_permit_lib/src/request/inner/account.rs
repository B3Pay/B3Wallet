use crate::{error::RequestError, request::ExecutionResult, request::RequestTrait};
use async_trait::async_trait;
use b3_helper_lib::environment::Environment;
use b3_wallet_lib::{
    account::WalletAccount,
    error::WalletError,
    store::{with_account_mut, with_wallet, with_wallet_mut},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// CREATE ACCOUNT
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct CreateAccount {
    name: Option<String>,
    env: Option<Environment>,
}

#[async_trait]
impl RequestTrait for CreateAccount {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let subaccount = with_wallet(|s| s.new_subaccount(self.env.clone()));

        let new_account: WalletAccount = subaccount.into();

        with_wallet_mut(|s| s.insert_account(new_account, self.name.clone()));

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        Ok(())
    }

    fn method_name(&self) -> String {
        "create_account".to_string()
    }
}

// REMOVE ACCOUNT
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct RemoveAccount {
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for RemoveAccount {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        with_wallet_mut(|s| s.remove_account(&self.account_id))?;

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        with_wallet(|s| {
            if s.account(&self.account_id).is_ok() {
                Ok(())
            } else {
                Err(RequestError::AccountDoesNotExist)
            }
        })
    }

    fn method_name(&self) -> String {
        "remove_account".to_string()
    }
}

// RENAME ACCOUNT
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct RenameAccount {
    pub new_name: String,
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for RenameAccount {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.rename(self.new_name.clone())
        })?;

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        with_wallet(|s| {
            if s.account(&self.new_name).is_ok() {
                Ok(())
            } else {
                Err(RequestError::AccountDoesNotExist)
            }
        })
    }

    fn method_name(&self) -> String {
        "rename_account".to_string()
    }
}

// HIDING ACCOUNT
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct HideAccount {
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for HideAccount {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.hide();
        })?;

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        with_account_mut(&self.account_id, |account| {
            if account.is_hidden() {
                Err(RequestError::AccountIsHidden)
            } else {
                Ok(())
            }
        })?
    }

    fn method_name(&self) -> String {
        "hide_account".to_string()
    }
}

// UNHIDING ACCOUNT
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UnhideAccount {
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for UnhideAccount {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.unhide();
        })?;

        Ok(self.into())
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        with_account_mut(&self.account_id, |account| {
            if !account.is_hidden() {
                Err(RequestError::AccountIsNotHidden)
            } else {
                Ok(())
            }
        })?
    }

    fn method_name(&self) -> String {
        "unhide_account".to_string()
    }
}

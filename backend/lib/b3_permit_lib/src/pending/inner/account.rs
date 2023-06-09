use crate::{
    error::RequestError,
    pending::RequestTrait,
    types::{ConsentInfo, ConsentMessageResponse},
};
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
pub struct CreateAccountRequest {
    name: Option<String>,
    env: Option<Environment>,
}

#[async_trait]
impl RequestTrait for CreateAccountRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let subaccount = with_wallet(|s| s.new_subaccount(self.env.clone()));

        let new_account: WalletAccount = subaccount.into();

        let name = new_account.id().to_string();

        with_wallet_mut(|s| s.insert_account(new_account, self.name.clone()));

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!("Account {} created", name),
        }))
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
pub struct RemoveAccountRequest {
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for RemoveAccountRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_wallet_mut(|s| s.remove_account(&self.account_id))?;

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!("Account {} removed", self.account_id),
        }))
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
pub struct RenameAccountRequest {
    pub new_name: String,
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for RenameAccountRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.rename(self.new_name.clone())
        })?;

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!("Account {} renamed to {}", self.account_id, self.new_name),
        }))
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
pub struct HideAccountRequest {
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for HideAccountRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.hide();
        })?;

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!("Account {} hidden", self.account_id),
        }))
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
pub struct UnhideAccountRequest {
    pub account_id: String,
}

#[async_trait]
impl RequestTrait for UnhideAccountRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.unhide();
        })?;

        Ok(ConsentMessageResponse::Valid(ConsentInfo {
            consent_message: format!("Account {} unhidden", self.account_id),
        }))
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

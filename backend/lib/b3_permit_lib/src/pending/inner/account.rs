use super::{InnerRequest, Request};
use crate::types::{ConsendInfo, ConsentMessageResponse};
use b3_helper_lib::types::Environment;
use b3_wallet_lib::{
    account::WalletAccount,
    error::WalletError,
    store::{with_account_mut, with_ledger, with_ledger_mut, with_wallet, with_wallet_mut},
};

use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// CREATE ACCOUNT
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct CreateAccountRequest {
    name: Option<String>,
    env: Option<Environment>,
}

impl From<CreateAccountRequest> for Request {
    fn from(args: CreateAccountRequest) -> Self {
        InnerRequest::CreateAccountRequest(args).into()
    }
}

impl CreateAccountRequest {
    pub fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let subaccount = with_wallet(|s| s.new_subaccount(self.env.clone()));

        let new_account: WalletAccount = subaccount.into();

        let name = new_account.id().to_string();

        with_wallet_mut(|s| s.insert_account(new_account, self.name.clone()));

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Account {} created", name),
            ..Default::default()
        }))
    }
}

// REMOVE ACCOUNT
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct RemoveAccountRequest {
    pub account_id: String,
}

impl From<RemoveAccountRequest> for Request {
    fn from(args: RemoveAccountRequest) -> Self {
        InnerRequest::RemoveAccountRequest(args).into()
    }
}

impl RemoveAccountRequest {
    pub fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_wallet_mut(|s| s.remove_account(&self.account_id))?;

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Account {} removed", self.account_id),
            ..Default::default()
        }))
    }
}

// RENAME ACCOUNT
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct RenameAccountRequest {
    pub new_name: String,
    pub account_id: String,
}

impl From<RenameAccountRequest> for Request {
    fn from(args: RenameAccountRequest) -> Self {
        InnerRequest::RenameAccountRequest(args).into()
    }
}

impl RenameAccountRequest {
    pub fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.rename(self.new_name.clone())
        })?;

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Account {} renamed to {}", self.account_id, self.new_name),
            ..Default::default()
        }))
    }
}

// HIDING ACCOUNT
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct HideAccountRequest {
    pub account_id: String,
}

impl From<HideAccountRequest> for Request {
    fn from(args: HideAccountRequest) -> Self {
        InnerRequest::HideAccountRequest(args).into()
    }
}

impl HideAccountRequest {
    pub fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.hide();
        })?;

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Account {} hidden", self.account_id),
            ..Default::default()
        }))
    }
}

// UNHIDING ACCOUNT
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct UnhideAccountRequest {
    pub account_id: String,
}

impl From<UnhideAccountRequest> for Request {
    fn from(args: UnhideAccountRequest) -> Self {
        InnerRequest::UnhideAccountRequest(args).into()
    }
}

impl UnhideAccountRequest {
    pub fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.unhide();
        })?;

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Account {} unhidden", self.account_id),
            ..Default::default()
        }))
    }
}

// ECDSA PUBLIC KEY
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EcdsaPublicKeyRequest {
    pub account_id: String,
}

impl From<EcdsaPublicKeyRequest> for Request {
    fn from(args: EcdsaPublicKeyRequest) -> Self {
        InnerRequest::EcdsaPublicKeyRequest(args).into()
    }
}

impl EcdsaPublicKeyRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        if ledger.is_ecdsa_set() {
            return Err(WalletError::EcdsaPublicKeyAlreadySet);
        }

        let ecdsa = ledger.ecdsa_public_key().await?;

        with_ledger_mut(&self.account_id, |ledger| ledger.set_ecdsa(ecdsa))??;

        Ok(ConsentMessageResponse::Valid(ConsendInfo {
            consent_message: format!("Ecdsa public key set for account {}", self.account_id),
            ..Default::default()
        }))
    }
}

use super::{InnerRequest, Request};
use crate::{
    account::WalletAccount,
    error::WalletError,
    store::{with_account_mut, with_ledger, with_ledger_mut, with_state, with_state_mut},
    types::SignedMessage,
};
use b3_helper::types::Environment;

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
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        let subaccount = with_state(|s| s.new_subaccount(self.env.clone()));

        let new_account: WalletAccount = subaccount.into();

        let account_id = with_state_mut(|s| s.insert_account(new_account, self.name.clone()));

        Ok(account_id.into())
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
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        with_state_mut(|s| s.remove_account(&self.account_id))?;

        Ok(SignedMessage::default())
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
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        let new_name = with_account_mut(&self.account_id, |account| {
            account.rename(self.new_name.clone())
        })?;

        Ok(new_name.into())
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
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.hide();
        })?;

        Ok(SignedMessage::default())
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
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        with_account_mut(&self.account_id, |account| {
            account.unhide();
        })?;

        Ok(SignedMessage::default())
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
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        if ledger.public_keys.is_ecdsa_set() {
            return Err(WalletError::EcdsaPublicKeyAlreadySet);
        }

        let ecdsa = ledger.ecdsa_public_key().await?;

        with_ledger_mut(&self.account_id, |ledger| {
            ledger.public_keys.set_ecdsa(ecdsa.clone())
        })??;

        Ok(SignedMessage::default())
    }
}

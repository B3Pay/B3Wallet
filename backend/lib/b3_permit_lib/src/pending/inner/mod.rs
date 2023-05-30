pub mod account;
pub mod setting;
pub mod signer;

use std::fmt;

use crate::types::ConsentMessageResponse;

use super::Request;
use account::{
    CreateAccountRequest, EcdsaPublicKeyRequest, HideAccountRequest, RemoveAccountRequest,
    RenameAccountRequest, UnhideAccountRequest,
};
use b3_wallet_lib::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use setting::{UpdateCanisterSettingsRequest, UpgradeCanisterRequest};
use signer::{AddSignerRequest, RemoveSignerRequest, UpdateSignerThresholdRequest};

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum InnerRequest {
    AddSignerRequest,
    RemoveSignerRequest,
    CreateAccountRequest,
    RemoveAccountRequest,
    RenameAccountRequest,
    HideAccountRequest,
    UnhideAccountRequest,
    EcdsaPublicKeyRequest,
    UpdateSignerThresholdRequest,
    UpdateCanisterSettingsRequest,
    UpgradeCanisterRequest,
}

impl InnerRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        match self {
            InnerRequest::AddSignerRequest(args) => args.execute(),
            InnerRequest::RemoveSignerRequest(args) => args.execute(),
            InnerRequest::CreateAccountRequest(args) => args.execute(),
            InnerRequest::RemoveAccountRequest(args) => args.execute(),
            InnerRequest::HideAccountRequest(args) => args.execute(),
            InnerRequest::UnhideAccountRequest(args) => args.execute(),
            InnerRequest::RenameAccountRequest(args) => args.execute(),
            InnerRequest::UpdateSignerThresholdRequest(args) => args.execute(),
            InnerRequest::EcdsaPublicKeyRequest(args) => args.execute().await,
            InnerRequest::UpdateCanisterSettingsRequest(args) => args.execute().await,
            InnerRequest::UpgradeCanisterRequest(args) => args.execute().await,
        }
    }
}

impl fmt::Display for InnerRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerRequest::AddSignerRequest(_) => write!(f, "AddSignerRequest"),
            InnerRequest::RemoveSignerRequest(_) => write!(f, "RemoveSignerRequest"),
            InnerRequest::CreateAccountRequest(_) => write!(f, "CreateAccountRequest"),
            InnerRequest::RemoveAccountRequest(_) => write!(f, "RemoveAccountRequest"),
            InnerRequest::HideAccountRequest(_) => write!(f, "HideAccountRequest"),
            InnerRequest::UnhideAccountRequest(_) => write!(f, "UnhideAccountRequest"),
            InnerRequest::RenameAccountRequest(_) => write!(f, "RenameAccountRequest"),
            InnerRequest::UpdateSignerThresholdRequest(_) => {
                write!(f, "UpdateSignerThresholdRequest")
            }
            InnerRequest::EcdsaPublicKeyRequest(_) => write!(f, "EcdsaPublicKeyRequest"),
            InnerRequest::UpdateCanisterSettingsRequest(_) => {
                write!(f, "UpdateCanisterSettingsRequest")
            }
            InnerRequest::UpgradeCanisterRequest(_) => write!(f, "UpgradeCanisterRequest"),
        }
    }
}

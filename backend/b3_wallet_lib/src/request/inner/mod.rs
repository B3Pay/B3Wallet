pub mod account;
pub mod setting;
pub mod signer;

use super::Request;
use crate::{error::WalletError, types::SignedMessage};
use account::{
    CreateAccountRequest, EcdsaPublicKeyRequest, HideAccountRequest, RemoveAccountRequest,
    RenameAccountRequest, UnhideAccountRequest,
};
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
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
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

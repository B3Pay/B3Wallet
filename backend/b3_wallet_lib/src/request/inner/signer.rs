use crate::{
    error::WalletError,
    request::Request,
    signer::{Roles, Signer},
    store::with_signers_mut,
    types::SignedMessage,
};
use b3_helper::types::{Metadata, SignerId};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use super::InnerRequest;

// ADD SIGNER
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct AddSignerRequest {
    pub name: Option<String>,
    pub role: Roles,
    pub signer_id: SignerId,
    pub expires_at: Option<u64>,
    pub threshold: Option<u8>,
}

impl From<&AddSignerRequest> for Signer {
    fn from(args: &AddSignerRequest) -> Self {
        Signer {
            name: args.name.clone(),
            role: args.role,
            threshold: args.threshold,
            expires_at: args.expires_at,
            metadata: Metadata::default(),
        }
    }
}

impl From<AddSignerRequest> for Request {
    fn from(args: AddSignerRequest) -> Self {
        InnerRequest::AddSignerRequest(args).into()
    }
}

impl AddSignerRequest {
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        let signer_id = self.signer_id.clone();
        with_signers_mut(|signers| {
            if signers.contains_key(&signer_id) {
                return Err(WalletError::SignerAlreadyExists(signer_id.to_string()));
            }

            signers.insert(signer_id.clone(), self.into());

            Ok(SignedMessage::default())
        })
    }
}

// REMOVE SIGNER
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct RemoveSignerRequest {
    pub signer_id: SignerId,
}

impl From<RemoveSignerRequest> for Request {
    fn from(args: RemoveSignerRequest) -> Self {
        InnerRequest::RemoveSignerRequest(args).into()
    }
}

impl RemoveSignerRequest {
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        let signer_id = self.signer_id.clone();
        with_signers_mut(|signers| {
            if !signers.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            signers.remove(&signer_id);

            Ok(SignedMessage::default())
        })
    }
}

// UPDATE SIGNER THRESHOLD
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct UpdateSignerThresholdRequest {
    pub signer_id: SignerId,
    pub threshold: u8,
}

impl From<UpdateSignerThresholdRequest> for Request {
    fn from(args: UpdateSignerThresholdRequest) -> Self {
        InnerRequest::UpdateSignerThresholdRequest(args).into()
    }
}

impl UpdateSignerThresholdRequest {
    pub fn execute(&self) -> Result<SignedMessage, WalletError> {
        let signer_id = self.signer_id.clone();
        with_signers_mut(|signers| {
            if !signers.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            let mut signer = signers.get_mut(&signer_id).unwrap();
            signer.threshold = Some(self.threshold);

            Ok(SignedMessage::default())
        })
    }
}

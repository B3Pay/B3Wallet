use crate::{
    error::RequestError,
    pending::RequestTrait,
    signer::{Roles, Signer},
    store::with_permit_mut,
    types::{ConsentInfo, ConsentMessageResponse},
};
use async_trait::async_trait;
use b3_helper_lib::types::{Metadata, SignerId};
use b3_wallet_lib::error::WalletError;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// ADD SIGNER
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
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

#[async_trait]
impl RequestTrait for AddSignerRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|link| {
            if link.signers.contains_key(&signer_id) {
                return Err(WalletError::SignerAlreadyExists(signer_id.to_string()));
            }

            link.signers.insert(signer_id.clone(), self.into());

            Ok(ConsentMessageResponse::Valid(ConsentInfo {
                consent_message: format!("Signer {} added", signer_id),
            }))
        })
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        if self.threshold.is_some() && self.role != Roles::Threshold {
            return Err(RequestError::InvalidThreshold);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "add_signer".to_string()
    }
}

// REMOVE SIGNER
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct RemoveSignerRequest {
    pub signer_id: SignerId,
}

#[async_trait]
impl RequestTrait for RemoveSignerRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|link| {
            if !link.signers.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            link.signers.remove(&signer_id);

            Ok(ConsentMessageResponse::Valid(ConsentInfo {
                consent_message: format!("Signer {} removed", signer_id),
            }))
        })
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        // check if the signer exists
        if !with_permit_mut(|link| link.signers.contains_key(&self.signer_id)) {
            return Err(RequestError::SignerDoesNotExist(self.signer_id.to_string()));
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "remove_signer".to_string()
    }
}

// UPDATE SIGNER THRESHOLD
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UpdateSignerThresholdRequest {
    pub signer_id: SignerId,
    pub threshold: u8,
}

#[async_trait]
impl RequestTrait for UpdateSignerThresholdRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|link| {
            if !link.signers.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            let mut signer = link.signers.get_mut(&signer_id).unwrap();
            signer.threshold = Some(self.threshold);

            Ok(ConsentMessageResponse::Valid(ConsentInfo {
                consent_message: format!("Signer {} threshold updated", signer_id),
            }))
        })
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        if self.threshold == 0 {
            return Err(RequestError::InvalidThreshold);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "update_signer_threshold".to_string()
    }
}

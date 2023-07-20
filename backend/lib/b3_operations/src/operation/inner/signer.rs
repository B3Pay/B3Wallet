use crate::error::OperationError;
use crate::operation::result::OperationResult;
use crate::operation::OperationTrait;
use crate::signer::roles::SignerRoles;
use crate::signer::signer::Signer;
use crate::store::with_permit;
use crate::store::with_permit_mut;
use async_trait::async_trait;
use b3_utils::types::{Metadata, SignerId};
use b3_wallet_lib::error::WalletError;
use candid::{CandidType, Deserialize};

// ADD SIGNER
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct AddSigner {
    pub name: String,
    pub role: SignerRoles,
    pub signer_id: SignerId,
    pub expires_at: Option<u64>,
    pub threshold: Option<u8>,
}

impl From<&AddSigner> for Signer {
    fn from(args: &AddSigner) -> Self {
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
impl OperationTrait for AddSigner {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|state| {
            if state.signers.contains_key(&signer_id) {
                return Err(WalletError::SignerAlreadyExists(signer_id.to_string()));
            }

            let signer = Signer::from(&self);

            state.signers.insert(signer_id, signer);

            Ok(self.into())
        })
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.threshold.is_some() {
            return Err(OperationError::InvalidThreshold);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "add_signer".to_string()
    }

    fn title(&self) -> String {
        format!("Add signer {}", self.name)
    }

    fn message(&self) -> String {
        format!("Add signer {}", self.name)
    }
}

// REMOVE SIGNER
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct RemoveSigner {
    pub signer_id: SignerId,
}

#[async_trait]
impl OperationTrait for RemoveSigner {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let signer_id = self.signer_id.clone();
        with_permit_mut(|permit| {
            if !permit.signers.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            permit.signers.remove(&signer_id);

            Ok(self.into())
        })
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        // check if the signer exists
        if !with_permit(|permit| permit.signers.contains_key(&self.signer_id)) {
            return Err(OperationError::SignerDoesNotExist(
                self.signer_id.to_string(),
            ));
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "remove_signer".to_string()
    }

    fn title(&self) -> String {
        format!("Remove signer {}", self.signer_id)
    }

    fn message(&self) -> String {
        format!("Remove signer {}", self.signer_id)
    }
}

// UPDATE SIGNER THRESHOLD
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct UpdateSignerThreshold {
    pub signer_id: SignerId,
    pub threshold: u8,
}

#[async_trait]
impl OperationTrait for UpdateSignerThreshold {
    async fn execute(self) -> Result<OperationResult, WalletError> {
        let signer_id = self.signer_id.clone();

        with_permit_mut(|state| {
            if !state.signers.contains_key(&signer_id) {
                return Err(WalletError::SignerDoesNotExist(signer_id.to_string()));
            }

            let mut signer = state.signers.get_mut(&signer_id).unwrap();
            signer.threshold = Some(self.threshold);

            Ok(self.into())
        })
    }

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.threshold == 0 {
            return Err(OperationError::InvalidThreshold);
        }

        with_permit(|state| {
            if !state.signers.contains_key(&self.signer_id) {
                return Err(OperationError::SignerDoesNotExist(
                    self.signer_id.to_string(),
                ));
            }
            return Ok(());
        })
    }

    fn method_name(&self) -> String {
        "update_signer_threshold".to_string()
    }

    fn title(&self) -> String {
        format!("Update signer {} threshold", self.signer_id)
    }

    fn message(&self) -> String {
        format!("Update signer {} threshold", self.signer_id)
    }
}

use b3_helper_lib::types::SignerId;

use crate::{error::PermitError, state::PrmitState, types::SignerMap};

use super::Signer;

impl PrmitState {
    pub fn signers(&self) -> SignerMap {
        self.signers.clone()
    }

    pub fn signer(&self, signer_id: &SignerId) -> Result<&Signer, PermitError> {
        self.signers
            .get(signer_id)
            .ok_or(PermitError::SignerNotFound(signer_id.to_string()))
    }

    pub fn signer_mut(&mut self, signer_id: &SignerId) -> Result<&mut Signer, PermitError> {
        self.signers
            .get_mut(signer_id)
            .ok_or(PermitError::SignerNotFound(signer_id.to_string()))
    }
}

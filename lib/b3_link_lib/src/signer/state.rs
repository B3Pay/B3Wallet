use b3_helper_lib::types::SignerId;

use crate::{error::RequestError, state::LinkState, types::SignerMap};

use super::Signer;

impl LinkState {
    pub fn signers(&self) -> SignerMap {
        self.signers.clone()
    }

    pub fn signer(&self, signer_id: &SignerId) -> Result<&Signer, RequestError> {
        self.signers
            .get(signer_id)
            .ok_or(RequestError::RequestNotExists)
    }

    pub fn signer_mut(&mut self, signer_id: &SignerId) -> Result<&mut Signer, RequestError> {
        self.signers
            .get_mut(signer_id)
            .ok_or(RequestError::RequestNotExists)
    }
}

use crate::{
    error::OperationError,
    nonce::RequestNonce,
    processed::ProcessedOperation,
    signer::Signer,
    types::{PendingRequestMap, ProcessedRequestList, ProcessedRequestMap, SignerMap},
};
use b3_utils::types::{OperationId, SignerId};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct OperationState {
    pub counters: RequestNonce,
    pub signers: SignerMap,
    pub pending: PendingRequestMap,
    pub processed: ProcessedRequestMap,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            signers: SignerMap::new(),
            pending: PendingRequestMap::new(),
            processed: ProcessedRequestMap::new(),
            counters: RequestNonce::new(),
        }
    }
}

impl OperationState {
    pub fn init_signers(&mut self, signers: SignerMap) {
        self.signers = signers;
    }

    pub fn add_signer(&mut self, signer_id: SignerId, signer: Signer) {
        self.signers.insert(signer_id, signer);
    }

    pub fn remove_signer(&mut self, signer_id: &SignerId) {
        self.signers.remove(signer_id);
    }

    pub fn signer(&self, signer_id: &SignerId) -> Result<&Signer, OperationError> {
        self.signers
            .get(signer_id)
            .ok_or(OperationError::SignerNotFound(signer_id.to_string()))
    }

    pub fn signer_mut(&mut self, signer_id: &SignerId) -> Result<&mut Signer, OperationError> {
        self.signers
            .get_mut(signer_id)
            .ok_or(OperationError::SignerNotFound(signer_id.to_string()))
    }

    pub fn signers(&self) -> &SignerMap {
        &self.signers
    }

    pub fn signers_mut(&mut self) -> &mut SignerMap {
        &mut self.signers
    }

    pub fn pending(&self) -> &PendingRequestMap {
        &self.pending
    }

    pub fn pending_mut(&mut self) -> &mut PendingRequestMap {
        &mut self.pending
    }

    pub fn processed_list(&self) -> ProcessedRequestList {
        self.processed
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    }

    pub fn insert_processed(
        &mut self,
        request_id: OperationId,
        processed: ProcessedOperation,
    ) -> Result<(), OperationError> {
        self.pending
            .remove(&request_id)
            .ok_or(OperationError::RequestNotFound(request_id))?;

        self.processed.insert(request_id, processed);

        Ok(())
    }

    pub fn processed(
        &self,
        request_id: &OperationId,
    ) -> Result<&ProcessedOperation, OperationError> {
        self.processed
            .get(&request_id)
            .ok_or(OperationError::RequestNotFound(request_id.to_owned()))
    }

    pub fn processed_mut(&mut self) -> &mut ProcessedRequestMap {
        &mut self.processed
    }

    pub fn counters(&self) -> &RequestNonce {
        &self.counters
    }

    pub fn counters_mut(&mut self) -> &mut RequestNonce {
        &mut self.counters
    }
}

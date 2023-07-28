use crate::{
    error::OperationError,
    processed::ProcessedOperation,
    types::{PendingOperationMap, ProcessedOperationMap, ProcessedOperations},
};
use b3_utils::{types::OperationId, Nonce};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct OperationState {
    pub(crate) pending: PendingOperationMap,
    pub(crate) processed: ProcessedOperationMap,
    pub(crate) nonce: Nonce,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            pending: PendingOperationMap::new(),
            processed: ProcessedOperationMap::new(),
            nonce: Nonce::new(),
        }
    }
}

impl OperationState {
    pub fn pending_map(&self) -> &PendingOperationMap {
        &self.pending
    }

    pub fn pending_map_mut(&mut self) -> &mut PendingOperationMap {
        &mut self.pending
    }

    pub fn processed_list(&self) -> ProcessedOperations {
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

    pub fn processed_mut(&mut self) -> &mut ProcessedOperationMap {
        &mut self.processed
    }
}

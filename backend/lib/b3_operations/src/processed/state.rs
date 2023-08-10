use crate::{
    error::OperationError,
    processed::ProcessedOperation,
    types::{ProcessedOperationMap, ProcessedOperations},
};
use b3_utils::types::OperationId;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct ProccessedState {
    pub(crate) processed: ProcessedOperationMap,
}

impl Default for ProccessedState {
    fn default() -> Self {
        ProccessedState {
            processed: ProcessedOperationMap::new(),
        }
    }
}

impl ProccessedState {
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
        self.processed
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

    pub fn check_request(&self, request_id: &OperationId) -> Result<(), OperationError> {
        if self.processed.get(request_id).is_some() {
            return Err(OperationError::RequestAlreadyProcessed(
                request_id.to_owned(),
            ));
        }

        if !self.processed.contains_key(&request_id) {
            return Err(OperationError::RequestNotFound(request_id.to_owned()));
        }

        Ok(())
    }
}

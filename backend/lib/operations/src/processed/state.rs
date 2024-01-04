use crate::{
    error::OperationError,
    processed::ProcessedOperation,
    types::{ProcessedOperationMap, ProcessedOperations},
};
use b3_utils::types::OperationId;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct ProccessedState {
    processeds: ProcessedOperationMap,
}

impl Default for ProccessedState {
    fn default() -> Self {
        ProccessedState {
            processeds: ProcessedOperationMap::new(),
        }
    }
}

impl ProccessedState {
    pub fn processed_list(&self) -> ProcessedOperations {
        self.processeds
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    }

    pub fn add(&mut self, operation_id: OperationId, processed: ProcessedOperation) {
        self.processeds.insert(operation_id, processed);
    }

    pub fn processed(
        &self,
        operation_id: &OperationId,
    ) -> Result<&ProcessedOperation, OperationError> {
        self.processeds
            .get(&operation_id)
            .ok_or(OperationError::RequestNotFound(operation_id.to_owned()))
    }

    pub fn processed_mut(&mut self) -> &mut ProcessedOperationMap {
        &mut self.processeds
    }

    pub fn check_request(&self, operation_id: &OperationId) -> Result<(), OperationError> {
        if self.processeds.get(operation_id).is_some() {
            return Err(OperationError::RequestAlreadyProcessed(
                operation_id.to_owned(),
            ));
        }

        if !self.processeds.contains_key(&operation_id) {
            return Err(OperationError::RequestNotFound(operation_id.to_owned()));
        }

        Ok(())
    }

    pub fn processeds(&self) -> &ProcessedOperationMap {
        &self.processeds
    }

    pub fn processeds_mut(&mut self) -> &mut ProcessedOperationMap {
        &mut self.processeds
    }
}

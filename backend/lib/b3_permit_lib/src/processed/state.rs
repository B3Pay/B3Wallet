use b3_helper_lib::types::RequestId;

use super::ProcessedRequest;
use crate::{error::PermitError, state::PrmitState, types::ProcessedRequestList};

impl PrmitState {
    pub fn processed_list(&self) -> ProcessedRequestList {
        self.processed
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    }

    pub fn insert_processed(
        &mut self,
        request_id: RequestId,
        processed: ProcessedRequest,
    ) -> Result<(), PermitError> {
        self.pending
            .remove(&request_id)
            .ok_or(PermitError::RequestNotFound(request_id))?;

        self.processed.insert(request_id, processed);

        Ok(())
    }

    pub fn processed(&self, request_id: &RequestId) -> Result<&ProcessedRequest, PermitError> {
        self.processed
            .get(&request_id)
            .ok_or(PermitError::RequestNotFound(request_id.to_owned()))
    }
}

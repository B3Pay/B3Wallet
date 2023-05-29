use b3_helper_lib::types::RequestId;

use super::ProcessedRequest;
use crate::{error::RequestError, state::PrmitState};

impl PrmitState {
    pub fn insert_processed(
        &mut self,
        request_id: RequestId,
        processed: ProcessedRequest,
    ) -> Result<(), RequestError> {
        self.pending
            .remove(&request_id)
            .ok_or(RequestError::RequestNotExists)?;

        self.processed.insert(request_id, processed);

        Ok(())
    }

    pub fn processed(&self, request_id: &RequestId) -> Result<&ProcessedRequest, RequestError> {
        self.processed
            .get(&request_id)
            .ok_or(RequestError::RequestNotExists)
    }
}

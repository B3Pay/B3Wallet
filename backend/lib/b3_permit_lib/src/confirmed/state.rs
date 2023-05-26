use b3_helper_lib::types::RequestId;

use super::ConfirmedRequest;
use crate::{error::RequestError, state::PrmitState};

impl PrmitState {
    pub fn insert_confirmed(
        &mut self,
        request_id: RequestId,
        confirmed: ConfirmedRequest,
    ) -> Result<(), RequestError> {
        self.pending
            .remove(&request_id)
            .ok_or(RequestError::RequestNotExists)?;

        self.confirmed.insert(request_id, confirmed);

        Ok(())
    }

    pub fn confirmed(&self, request_id: &RequestId) -> Result<&ConfirmedRequest, RequestError> {
        self.confirmed
            .get(&request_id)
            .ok_or(RequestError::RequestNotExists)
    }
}

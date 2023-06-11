use b3_helper_lib::types::RequestId;

use super::new::{PendingRequest, RequestArgs};
use crate::{error::RequestError, state::PrmitState, types::PendingRequestList};

impl PrmitState {
    pub fn new_request(&self, args: RequestArgs) -> PendingRequest {
        let id = self.request_counter();

        PendingRequest::new(id, args)
    }

    pub fn insert_new_request(&mut self, sign_request: PendingRequest) -> RequestId {
        let id = sign_request.id;

        self.pending.insert(id.clone(), sign_request);

        self.counters.increment_request();

        id
    }

    pub fn request_counter(&self) -> usize {
        self.counters.request()
    }

    pub fn remove_request(&mut self, request_id: &RequestId) {
        self.pending.remove(request_id);
    }

    pub fn pending_list(&self) -> PendingRequestList {
        self.pending
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    }

    pub fn request(&self, request_id: &RequestId) -> Result<&PendingRequest, RequestError> {
        self.pending
            .get(request_id)
            .ok_or(RequestError::RequestNotFound(request_id.to_owned()))
    }

    pub fn request_mut(
        &mut self,
        request_id: &RequestId,
    ) -> Result<&mut PendingRequest, RequestError> {
        self.pending
            .get_mut(request_id)
            .ok_or(RequestError::RequestNotFound(request_id.to_owned()))
    }

    pub fn check_request(&self, request_id: &RequestId) -> Result<(), RequestError> {
        if self.processed.get(request_id).is_some() {
            return Err(RequestError::RequestAlreadyProcessed(request_id.to_owned()));
        }

        if !self.pending.contains_key(&request_id) {
            return Err(RequestError::RequestNotFound(request_id.to_owned()));
        }

        Ok(())
    }
}

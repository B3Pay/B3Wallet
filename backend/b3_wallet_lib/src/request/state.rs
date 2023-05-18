use super::{PendingRequest, RequestArgs};
use crate::{
    error::WalletError,
    state::State,
    types::{PendingRequestList, RequestId},
};

impl State {
    pub fn new_request(&self, args: RequestArgs) -> PendingRequest {
        let id = self.request_counter();

        PendingRequest::new(id, args)
    }

    pub fn insert_new_request(&mut self, sign_request: PendingRequest) -> RequestId {
        let id = sign_request.id();

        self.pending_requests.insert(id.clone(), sign_request);

        self.counters.increment_request();

        id
    }

    pub fn request_counter(&self) -> usize {
        self.counters.request()
    }

    pub fn remove_request(&mut self, request_id: RequestId) {
        self.pending_requests.remove(&request_id);
    }

    pub fn requests(&self) -> PendingRequestList {
        self.pending_requests
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    }

    pub fn request(&self, request_id: RequestId) -> Result<&PendingRequest, WalletError> {
        self.pending_requests
            .get(&request_id)
            .ok_or(WalletError::RequestNotFound(request_id))
    }

    pub fn request_mut(
        &mut self,
        request_id: RequestId,
    ) -> Result<&mut PendingRequest, WalletError> {
        self.pending_requests
            .get_mut(&request_id)
            .ok_or(WalletError::RequestNotFound(request_id))
    }

    pub fn check_request(&self, request_id: RequestId) -> Result<(), WalletError> {
        if self.confirmed_request(request_id).is_ok() {
            return Err(WalletError::RequestAlreadyConfirmed(request_id));
        }

        if !self.pending_requests.contains_key(&request_id) {
            return Err(WalletError::RequestNotFound(request_id));
        }

        Ok(())
    }
}

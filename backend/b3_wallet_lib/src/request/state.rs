use super::{Request, RequestArgs};
use crate::{
    error::WalletError,
    state::State,
    types::{PendingRequestMap, RequestId},
};

impl State {
    pub fn new_request(&self, args: RequestArgs, deadline: Option<u64>) -> Request {
        let id = self.request_counter();

        Request::new(id, args, deadline)
    }

    pub fn insert_request(&mut self, sign_request: Request) -> RequestId {
        let id = sign_request.id();

        self.pending_requests.insert(id.clone(), sign_request);

        id
    }

    pub fn request_counter(&self) -> usize {
        self.counters.request
    }

    pub fn remove_request(&mut self, request_id: RequestId) {
        self.pending_requests.remove(&request_id);
    }

    pub fn requests(&self) -> PendingRequestMap {
        self.pending_requests.clone()
    }

    pub fn request(&self, request_id: RequestId) -> Result<&Request, WalletError> {
        self.pending_requests
            .get(&request_id)
            .ok_or(WalletError::RequestNotFound(request_id))
    }

    pub fn request_mut(&mut self, request_id: RequestId) -> Result<&mut Request, WalletError> {
        self.pending_requests
            .get_mut(&request_id)
            .ok_or(WalletError::RequestNotFound(request_id))
    }

    pub fn check_request(&self, request_id: RequestId) -> Result<(), WalletError> {
        if self.confirmed(request_id).is_ok() {
            return Err(WalletError::RequestAlreadyConfirmed(request_id));
        }

        if !self.pending_requests.contains_key(&request_id) {
            return Err(WalletError::RequestNotFound(request_id));
        }

        Ok(())
    }
}

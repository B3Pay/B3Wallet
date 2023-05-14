use super::{sign::SignRequest, Request, RequestArgs};
use crate::{
    error::WalletError,
    signer::Roles,
    state::State,
    types::{RequestId, RequestMap},
};

impl State {
    pub fn new_request(&self, role: Roles, request: SignRequest, deadline: Option<u64>) -> Request {
        let id = self.request_counter();

        let request_args = RequestArgs {
            id,
            request,
            deadline,
            allowed_role: role,
        };

        request_args.into()
    }

    pub fn insert_request(&mut self, sign_request: Request) -> RequestId {
        let id = sign_request.id();

        self.requests.insert(id.clone(), sign_request);

        id
    }

    pub fn request_counter(&self) -> usize {
        self.counters.request
    }

    pub fn remove_request(&mut self, request_id: RequestId) {
        self.requests.remove(&request_id);
    }

    pub fn requests(&self) -> RequestMap {
        self.requests.clone()
    }

    pub fn request(&self, request_id: RequestId) -> Result<&Request, WalletError> {
        self.requests
            .get(&request_id)
            .ok_or(WalletError::RequestNotFound(request_id))
    }

    pub fn request_mut(&mut self, request_id: RequestId) -> Result<&mut Request, WalletError> {
        self.requests
            .get_mut(&request_id)
            .ok_or(WalletError::RequestNotFound(request_id))
    }

    pub fn check_request(&self, request_id: RequestId) -> Result<(), WalletError> {
        if self.confirmed(request_id).is_ok() {
            return Err(WalletError::RequestAlreadyConfirmed(request_id));
        }

        if !self.requests.contains_key(&request_id) {
            return Err(WalletError::RequestNotFound(request_id));
        }

        Ok(())
    }
}

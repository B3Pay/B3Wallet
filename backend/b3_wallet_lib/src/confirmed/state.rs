use super::ConfirmedRequest;
use crate::{
    counter::WalletCounters,
    error::WalletError,
    state::State,
    types::{ConfirmedRequestMap, RequestId},
};
use std::borrow::{Borrow, BorrowMut};

impl State {
    pub fn insert_confirmed_request(
        &mut self,
        request_id: RequestId,
        confirmed: ConfirmedRequest,
    ) -> Result<(), WalletError> {
        self.pending_requests
            .remove(&request_id)
            .ok_or(WalletError::RequestNotExists)?;

        self.confirmed_requests.insert(request_id, confirmed);

        Ok(())
    }

    pub fn confirmed_request(
        &self,
        request_id: RequestId,
    ) -> Result<&ConfirmedRequest, WalletError> {
        self.confirmed_requests
            .get(&request_id)
            .ok_or(WalletError::RequestNotExists)
    }

    pub fn confirmed_requests(&self) -> &ConfirmedRequestMap {
        self.confirmed_requests.borrow()
    }

    pub fn confirmed_requests_mut(&mut self) -> &mut ConfirmedRequestMap {
        self.confirmed_requests.borrow_mut()
    }

    pub fn insert_confirmed(&mut self, request_id: RequestId, confiremd: ConfirmedRequest) {
        self.confirmed_requests.insert(request_id, confiremd);
    }

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.pending_requests.clear();
        self.confirmed_requests.clear();
        self.counters = WalletCounters::new();

        self.init_wallet();
    }
}

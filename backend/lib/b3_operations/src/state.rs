use crate::{
    error::OperationError,
    nonce::RequestNonce,
    processed::ProcessedOperation,
    types::{PendingRequestMap, ProcessedRequestList, ProcessedRequestMap, UserMap},
    user::UserState,
};
use b3_utils::types::{OperationId, UserId};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct OperationState {
    pub counters: RequestNonce,
    pub users: UserMap,
    pub pending: PendingRequestMap,
    pub processed: ProcessedRequestMap,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            users: UserMap::new(),
            pending: PendingRequestMap::new(),
            processed: ProcessedRequestMap::new(),
            counters: RequestNonce::new(),
        }
    }
}

impl OperationState {
    pub fn init_users(&mut self, signers: UserMap) {
        self.users = signers;
    }

    pub fn add_user(&mut self, user_id: UserId, user: UserState) {
        self.users.insert(user_id, user);
    }

    pub fn remove_user(&mut self, user_id: &UserId) {
        self.users.remove(user_id);
    }

    pub fn user(&self, user_id: &UserId) -> Result<&UserState, OperationError> {
        self.users
            .get(user_id)
            .ok_or(OperationError::UserNotFound(user_id.to_string()))
    }

    pub fn user_mut(&mut self, user_id: &UserId) -> Result<&mut UserState, OperationError> {
        self.users
            .get_mut(user_id)
            .ok_or(OperationError::UserNotFound(user_id.to_string()))
    }

    pub fn users(&self) -> &UserMap {
        &self.users
    }

    pub fn users_mut(&mut self) -> &mut UserMap {
        &mut self.users
    }

    pub fn pending(&self) -> &PendingRequestMap {
        &self.pending
    }

    pub fn pending_mut(&mut self) -> &mut PendingRequestMap {
        &mut self.pending
    }

    pub fn processed_list(&self) -> ProcessedRequestList {
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
        self.pending
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

    pub fn processed_mut(&mut self) -> &mut ProcessedRequestMap {
        &mut self.processed
    }

    pub fn counters(&self) -> &RequestNonce {
        &self.counters
    }

    pub fn counters_mut(&mut self) -> &mut RequestNonce {
        &mut self.counters
    }
}

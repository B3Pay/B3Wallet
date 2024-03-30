use crate::{
    error::OperationError,
    pending::{PendingOperation, RequestArgs},
    types::{PendingOperationMap, PendingOperations},
};
use b3_utils::{nonce::Nonce, principal::StoredPrincipal, types::OperationId};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct OperationState {
    pendings: PendingOperationMap,
    nonce: Nonce,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            pendings: PendingOperationMap::new(),
            nonce: Nonce::new(None),
        }
    }
}

impl OperationState {
    pub fn new_request(&self, signer_id: StoredPrincipal, args: RequestArgs) -> PendingOperation {
        let id = self.request_counter();

        PendingOperation::new(id, signer_id, args)
    }

    pub fn add(&mut self, sign_request: PendingOperation) -> OperationId {
        let id = sign_request.id;

        self.pendings.insert(id.clone(), sign_request);

        self.nonce.increment();

        id
    }

    pub fn request_counter(&self) -> u64 {
        self.nonce.get()
    }

    pub fn remove_request(&mut self, request_id: &OperationId) {
        self.pendings.remove(request_id);
    }

    pub fn pending_list(&self) -> PendingOperations {
        self.pendings
            .iter()
            .map(|(_, request)| request.clone())
            .collect()
    }

    pub fn pending(&self, request_id: &OperationId) -> Result<&PendingOperation, OperationError> {
        self.pendings
            .get(request_id)
            .ok_or(OperationError::RequestNotFound(request_id.to_owned()))
    }

    pub fn request_mut(
        &mut self,
        request_id: &OperationId,
    ) -> Result<&mut PendingOperation, OperationError> {
        self.pendings
            .get_mut(request_id)
            .ok_or(OperationError::RequestNotFound(request_id.to_owned()))
    }

    pub fn pending_map(&self) -> &PendingOperationMap {
        &self.pendings
    }

    pub fn pending_map_mut(&mut self) -> &mut PendingOperationMap {
        &mut self.pendings
    }
}

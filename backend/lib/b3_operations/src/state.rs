use crate::types::PendingOperationMap;
use b3_utils::nonce::Nonce;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct OperationState {
    pub(crate) pending: PendingOperationMap,
    pub(crate) nonce: Nonce,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            pending: PendingOperationMap::new(),
            nonce: Nonce::new(None),
        }
    }
}

impl OperationState {
    pub fn pending_map(&self) -> &PendingOperationMap {
        &self.pending
    }

    pub fn pending_map_mut(&mut self) -> &mut PendingOperationMap {
        &mut self.pending
    }
}

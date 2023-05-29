use crate::{
    counter::RequestCounters,
    types::{PendingRequestMap, ProcessedRequestMap, SignerMap},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct PrmitState {
    pub counters: RequestCounters,
    pub signers: SignerMap,
    pub pending: PendingRequestMap,
    pub processed: ProcessedRequestMap,
}

impl Default for PrmitState {
    fn default() -> Self {
        PrmitState {
            signers: SignerMap::new(),
            pending: PendingRequestMap::new(),
            processed: ProcessedRequestMap::new(),
            counters: RequestCounters::new(),
        }
    }
}

impl PrmitState {}

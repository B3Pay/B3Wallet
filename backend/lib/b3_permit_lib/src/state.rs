use crate::{
    counter::RequestCounters,
    types::{ConfirmedRequestMap, PendingRequestMap, SignerMap},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct PrmitState {
    pub counters: RequestCounters,
    pub signers: SignerMap,
    pub pending: PendingRequestMap,
    pub confirmed: ConfirmedRequestMap,
}

impl Default for PrmitState {
    fn default() -> Self {
        PrmitState {
            signers: SignerMap::new(),
            pending: PendingRequestMap::new(),
            confirmed: ConfirmedRequestMap::new(),
            counters: RequestCounters::new(),
        }
    }
}

impl PrmitState {}

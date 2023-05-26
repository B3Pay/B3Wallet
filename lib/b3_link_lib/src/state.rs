use crate::{
    counter::RequestCounters,
    types::{ConfirmedRequestMap, PendingRequestMap, SignerMap},
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct LinkState {
    pub counters: RequestCounters,
    pub signers: SignerMap,
    pub pending: PendingRequestMap,
    pub confirmed: ConfirmedRequestMap,
}

impl Default for LinkState {
    fn default() -> Self {
        LinkState {
            signers: SignerMap::new(),
            pending: PendingRequestMap::new(),
            confirmed: ConfirmedRequestMap::new(),
            counters: RequestCounters::new(),
        }
    }
}

impl LinkState {}

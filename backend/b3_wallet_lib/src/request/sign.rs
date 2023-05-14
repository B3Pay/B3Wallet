use super::{
    bitcoin::SendBitcoinRequest, evm::EvmSignRequest, icp::SendIcpRequest,
    inner::InnerCanisterRequest,
};
use b3_helper::types::CanisterId;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize)]
pub enum SignRequest {
    Evm(EvmSignRequest),
    Bitcoin(SendBitcoinRequest),
    Icp(SendIcpRequest),
    InnerCanister(InnerCanisterRequest),
}

impl SignRequest {
    pub fn new_evm(hex_raw_tx: Vec<u8>, chain_id: u64, deadline: Option<u64>) -> Self {
        SignRequest::Evm(EvmSignRequest::new(hex_raw_tx, chain_id, deadline))
    }

    pub fn new_bitcoin(amount: u64, address: String, deadline: Option<u64>) -> Self {
        SignRequest::Bitcoin(SendBitcoinRequest::new(amount, address, deadline))
    }

    pub fn new_icp(amount: u64, to: CanisterId, deadline: Option<u64>) -> Self {
        SignRequest::Icp(SendIcpRequest::new(amount, to, deadline))
    }

    pub fn new_inner_canister(inner_canister_request: InnerCanisterRequest) -> Self {
        SignRequest::InnerCanister(inner_canister_request)
    }
}

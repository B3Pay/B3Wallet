use super::{
    bitcoin::SendBitcoinRequest, evm::EvmSignRequest, icp::SendIcpRequest,
    inner::InnerCanisterRequest,
};
use b3_helper::types::CanisterId;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

pub trait SignRequestTrait {
    fn get_id(&self) -> String;
    fn get_deadline(&self) -> u64;
}

#[derive(CandidType, Clone, Deserialize)]
pub enum SignRequest {
    Evm(EvmSignRequest),
    Bitcoin(SendBitcoinRequest),
    Icp(SendIcpRequest),
    InnerCanister(InnerCanisterRequest),
}

impl SignRequestTrait for SignRequest {
    fn get_id(&self) -> String {
        match self {
            SignRequest::Evm(evm_sign_request) => evm_sign_request.id.clone(),
            SignRequest::Bitcoin(send_bitcoin_request) => send_bitcoin_request.id.clone(),
            SignRequest::Icp(send_icp_request) => send_icp_request.id.clone(),
            SignRequest::InnerCanister(inner_canister_request) => inner_canister_request.get_id(),
        }
    }

    fn get_deadline(&self) -> u64 {
        match self {
            SignRequest::Evm(evm_sign_request) => evm_sign_request.deadline,
            SignRequest::Bitcoin(send_bitcoin_request) => send_bitcoin_request.deadline,
            SignRequest::Icp(send_icp_request) => send_icp_request.deadline,
            SignRequest::InnerCanister(inner_canister_request) => {
                inner_canister_request.get_deadline()
            }
        }
    }
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

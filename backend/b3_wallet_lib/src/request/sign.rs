use crate::error::WalletError;

use super::{
    bitcoin::SendBitcoinRequest, evm::EvmSignRequest, icp::SendIcpRequest,
    inter::InterCanisterRequest, Executable,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize)]
pub enum SignRequest {
    Evm(EvmSignRequest),
    Bitcoin(SendBitcoinRequest),
    Icp(SendIcpRequest),
    InnerCanister(InterCanisterRequest),
}

impl Executable for SignRequest {
    fn execute(&self) -> Result<(), WalletError> {
        match self {
            SignRequest::InnerCanister(inner_canister_request) => inner_canister_request.execute(),
            SignRequest::Evm(evm_sign_request) => evm_sign_request.execute(),
            SignRequest::Bitcoin(send_bitcoin_request) => send_bitcoin_request.execute(),
            SignRequest::Icp(send_icp_request) => send_icp_request.execute(),
        }
    }
}

impl From<InterCanisterRequest> for SignRequest {
    fn from(inter_canister_request: InterCanisterRequest) -> Self {
        SignRequest::InnerCanister(inter_canister_request)
    }
}

impl From<EvmSignRequest> for SignRequest {
    fn from(evm_sign_request: EvmSignRequest) -> Self {
        SignRequest::Evm(evm_sign_request)
    }
}

impl From<SendBitcoinRequest> for SignRequest {
    fn from(send_bitcoin_request: SendBitcoinRequest) -> Self {
        SignRequest::Bitcoin(send_bitcoin_request)
    }
}

impl From<SendIcpRequest> for SignRequest {
    fn from(send_icp_request: SendIcpRequest) -> Self {
        SignRequest::Icp(send_icp_request)
    }
}

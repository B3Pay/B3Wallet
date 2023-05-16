use super::{bitcoin::SendBitcoinRequest, evm::EvmSignRequest, icp::SendIcpRequest, inter::*};
use crate::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[enum_dispatch(SignRequest, InterCanisterRequest)]
pub trait Executable {
    fn execute(&self) -> Result<(), WalletError>;
}

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize)]
pub enum SignRequest {
    EvmSignRequest,
    SendBitcoinRequest,
    SendIcpRequest,
    InterCanisterRequest,
}

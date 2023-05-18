pub mod other;
pub mod sign;
pub mod transfer;

use crate::{error::WalletError, types::SignedMessage};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use other::*;
use sign::*;
use transfer::*;

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum EvmRequest {
    EvmTransferEthRequest,
    EvmTransferErc20Request,
    EvmDeployContractRequest,
    EvmSignMessageRequest,
    EvmSignTranscationRequest,
    EvmSignRawTransactionRequest,
}

impl EvmRequest {
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
        match self {
            EvmRequest::EvmTransferEthRequest(args) => args.execute().await,
            EvmRequest::EvmTransferErc20Request(args) => args.execute().await,
            EvmRequest::EvmDeployContractRequest(args) => args.execute().await,
            EvmRequest::EvmSignTranscationRequest(args) => args.execute().await,
            EvmRequest::EvmSignMessageRequest(args) => args.execute().await,
            EvmRequest::EvmSignRawTransactionRequest(args) => args.execute().await,
        }
    }
}

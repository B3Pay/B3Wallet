pub mod other;
pub mod sign;
pub mod transfer;

use std::fmt;

use crate::types::ConsentMessageResponse;
use b3_wallet_lib::error::WalletError;
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
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
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

impl fmt::Display for EvmRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvmRequest::EvmTransferEthRequest(_) => write!(f, "EvmTransferEthRequest"),
            EvmRequest::EvmTransferErc20Request(_) => {
                write!(f, "EvmTransferErc20Request")
            }
            EvmRequest::EvmDeployContractRequest(_) => {
                write!(f, "EvmDeployContractRequest")
            }
            EvmRequest::EvmSignTranscationRequest(_) => {
                write!(f, "EvmSignTranscationRequest")
            }
            EvmRequest::EvmSignMessageRequest(_) => write!(f, "EvmSignMessageRequest"),
            EvmRequest::EvmSignRawTransactionRequest(_) => {
                write!(f, "EvmSignRawTransactionRequest")
            }
        }
    }
}

use super::btc::transfer::*;
use super::evm::other::*;
use super::evm::sign::*;
use super::evm::transfer::*;
use super::icp::transfer::*;
use super::inner::account::*;
use super::inner::setting::*;
use super::inner::signer::*;
use super::result::ExecutionResult;
use crate::error::PermitError;

use async_trait::async_trait;
use b3_wallet_lib::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[async_trait]
#[enum_dispatch]
pub trait RequestTrait {
    fn method_name(&self) -> String;
    fn validate_request(&self) -> Result<(), PermitError>;
    async fn execute(self) -> Result<ExecutionResult, WalletError>;
}

#[enum_dispatch(RequestTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Request {
    // EVM
    EvmTransfer,
    EvmSignMessage,
    EvmTransferErc20,
    EvmDeployContract,
    EvmSignTranscation,
    EvmSignRawTransaction,
    // BTC
    BtcTransfer,
    // ICP
    IcpTransfer,
    TopUpTransfer,
    // INNER
    AddSigner,
    RemoveSigner,
    CreateAccount,
    RemoveAccount,
    RenameAccount,
    HideAccount,
    UnhideAccount,
    UpgradeCanister,
    UpdateSignerThreshold,
    UpdateCanisterSettings,
}

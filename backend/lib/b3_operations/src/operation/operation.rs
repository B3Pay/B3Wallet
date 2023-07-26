use std::fmt;

use super::btc::*;
use super::evm::*;
use super::icp::*;
use super::inner::*;
use super::result::OperationResult;
use crate::error::OperationError;
use crate::operation::global::SendToken;

use async_trait::async_trait;
use b3_wallet_lib::error::WalletError;

use candid::{CandidType, Deserialize};
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch]
pub trait OperationTrait {
    fn title(&self) -> String;
    fn message(&self) -> String;
    fn method_name(&self) -> String;
    fn validate_request(&self) -> Result<(), OperationError>;
    async fn execute(self) -> Result<OperationResult, WalletError>;
}

#[enum_dispatch(OperationTrait)]
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub enum Operation {
    SendToken,
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

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

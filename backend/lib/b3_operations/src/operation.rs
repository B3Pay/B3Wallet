use std::fmt;

use crate::error::OperationError;
use async_trait::async_trait;
use b3_wallet_lib::error::WalletError;
use candid::{CandidType, Deserialize};
use enum_dispatch::enum_dispatch;

pub mod btc;
pub mod evm;
pub mod global;
pub mod icp;
pub mod inner;
pub mod result;

use btc::*;
use evm::*;
use icp::*;
use inner::*;

mod state;
pub use state::*;

use global::SendToken;
use result::OperationResult;

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
    AddUser,
    RemoveUser,
    CreateAccount,
    RemoveAccount,
    RenameAccount,
    HideAccount,
    UnhideAccount,
    UpgradeCanister,
    UpdateCanisterSettings,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

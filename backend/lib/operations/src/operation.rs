use std::fmt;

use crate::error::OperationError;
use async_trait::async_trait;
use b3wallet_lib::error::WalletError;
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

#[derive(CandidType, Deserialize, PartialEq, Debug, Clone)]
pub enum OperationEnum {
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

impl Operation {
    pub fn operation_enum(&self) -> OperationEnum {
        match self {
            Operation::SendToken(_) => OperationEnum::SendToken,
            // EVM
            Operation::EvmTransfer(_) => OperationEnum::EvmTransfer,
            Operation::EvmSignMessage(_) => OperationEnum::EvmSignMessage,
            Operation::EvmTransferErc20(_) => OperationEnum::EvmTransferErc20,
            Operation::EvmDeployContract(_) => OperationEnum::EvmDeployContract,
            Operation::EvmSignTranscation(_) => OperationEnum::EvmSignTranscation,
            Operation::EvmSignRawTransaction(_) => OperationEnum::EvmSignRawTransaction,
            // BTC
            Operation::BtcTransfer(_) => OperationEnum::BtcTransfer,
            // ICP
            Operation::IcpTransfer(_) => OperationEnum::IcpTransfer,
            Operation::TopUpTransfer(_) => OperationEnum::TopUpTransfer,
            // INNER
            Operation::AddUser(_) => OperationEnum::AddUser,
            Operation::RemoveUser(_) => OperationEnum::RemoveUser,
            Operation::CreateAccount(_) => OperationEnum::CreateAccount,
            Operation::RemoveAccount(_) => OperationEnum::RemoveAccount,
            Operation::RenameAccount(_) => OperationEnum::RenameAccount,
            Operation::HideAccount(_) => OperationEnum::HideAccount,
            Operation::UnhideAccount(_) => OperationEnum::UnhideAccount,
            Operation::UpgradeCanister(_) => OperationEnum::UpgradeCanister,
            Operation::UpdateCanisterSettings(_) => OperationEnum::UpdateCanisterSettings,
        }
    }
}

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

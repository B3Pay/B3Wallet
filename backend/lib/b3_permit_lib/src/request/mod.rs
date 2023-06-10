pub mod btc;
pub mod evm;
pub mod icp;
pub mod inner;
pub mod success;

use b3_helper_lib::types::Deadline;
use btc::transfer::*;
use evm::other::*;
use evm::sign::*;
use evm::transfer::*;
use icp::transfer::*;
use inner::account::*;
use inner::setting::*;
use inner::signer::*;

use crate::error::RequestError;
use crate::signer::Roles;

use async_trait::async_trait;
use b3_wallet_lib::error::WalletError;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use self::success::ExecutionResult;

#[async_trait]
#[enum_dispatch]
pub trait RequestTrait {
    fn method_name(&self) -> String;
    fn validate_request(&self) -> Result<(), RequestError>;
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
    TopUpCanister,
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

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct RequestArgs {
    pub role: Roles,
    pub request: Request,
    pub deadline: Option<Deadline>,
}

impl RequestArgs {
    pub fn new(role: Roles, request: Request, deadline: Option<Deadline>) -> Self {
        RequestArgs {
            role,
            request,
            deadline,
        }
    }
}

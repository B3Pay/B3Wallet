use std::fmt;

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
use crate::request::global::SendToken;

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

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::SendToken(SendToken {
                to,
                amount,
                account_id,
                chain,
            }) => write!(
                f,
                "SendToken {{ to: {}, amount: {}, account_id: {}, chain: {} }}",
                to, amount, account_id, chain
            ),
            Request::TopUpTransfer(TopUpTransfer {
                amount,
                account_id,
                canister_id,
                fee: _,
            }) => write!(
                f,
                "TopUpTransfer {{ amount: {}, account_id: {}, canister_id: {} }}",
                amount, account_id, canister_id
            ),
            Request::AddSigner(AddSigner {
                name,
                role,
                signer_id,
                expires_at: _,
                threshold: _,
            }) => write!(
                f,
                "AddSigner {{ name: {}, role: {}, signer_id: {} }}",
                name, role, signer_id
            ),
            Request::RemoveSigner(RemoveSigner { signer_id }) => {
                write!(f, "RemoveSigner {{ signer_id: {} }}", signer_id)
            }
            Request::UpgradeCanister(UpgradeCanister {
                wasm_hash_string,
                wasm_version,
            }) => {
                write!(
                    f,
                    "UpgradeCanister {{ wasm_hash_string: {}, wasm_version: {} }}",
                    wasm_hash_string, wasm_version
                )
            }
            Request::UpdateCanisterSettings(UpdateCanisterSettings {
                settings,
                canister_id,
            }) => {
                write!(
                    f,
                    "UpdateCanisterSettings {{ settings: {:?}, canister_id: {} }}",
                    settings, canister_id
                )
            }
            _ => write!(f, "Not Implemented yet"),
        }
    }
}

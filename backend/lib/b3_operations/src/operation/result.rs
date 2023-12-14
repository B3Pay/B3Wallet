use std::fmt;

use super::btc::transfer::BtcTransfer;
use super::evm::sign::{EvmSignMessage, EvmSignRawTransaction, EvmSignTranscation};
use super::evm::transfer::{EvmTransfer, EvmTransferErc20};
use super::global::SendToken;
use super::icp::transfer::{IcpTransfer, NotifyTopUp, TopUpTransfer};
use super::inner::account::{
    CreateAccount, HideAccount, RemoveAccount, RenameAccount, UnhideAccount,
};
use super::inner::setting::{UpdateCanisterSettings, UpgradeCanister};
use super::inner::user::{AddUser, RemoveUser};

use b3_utils::ledger::TransferBlockIndex;
use b3_wallet_lib::ledger::evm::london::EvmTransaction1559;
use b3_wallet_lib::ledger::types::SendResult;
use candid::{CandidType, Deserialize};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait ExecutionTrait {}

#[derive(CandidType, Clone, Deserialize, Debug)]
#[enum_dispatch(ExecutionTrait)]
pub enum OperationResult {
    Empty(Empty),
    TokenSent(TokenSent),
    IcpTransfered(IcpTransfered),
    EvmTransfered(EvmTransfered),
    EvmErc20Transfered(EvmErc20Transfered),
    TopUpTransfered(TopUpTransfered),
    CanisterTopUped(CanisterTopUped),
    BtcTransfered(BtcTransfered),
    SignerAdded(AddUser),
    SignerRemoved(RemoveUser),
    CanisterUpgraded(UpgradeCanister),
    CanisterSettingsUpdated(UpdateCanisterSettings),
    AccountCreated(CreateAccount),
    AccountRemoved(RemoveAccount),
    AccountRenamed(RenameAccount),
    AccountHidden(HideAccount),
    AccountUnhidden(UnhideAccount),
    EvmTransactionSigned(EvmTransactionSigned),
    EvmRawTransactionSigned(EvmRawTransactionSigned),
    EvmContractDeployed(EvmContractDeployed),
    EvmMessageSigned(EvmMessageSigned),
}

#[rustfmt::skip]
impl fmt::Display for OperationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationResult::Empty(_) => write!(f, "Empty"),
            OperationResult::TokenSent(TokenSent(ref args, ref tx_id)) => write!(f, "TokenSent: from {} in {} to {} at tx {}", args.account_id, args.chain, args.to, tx_id),
            OperationResult::IcpTransfered(IcpTransfered(args, block_index)) => write!(f, "IcpTransfered: from {} to {} at block {}", args.account_id, args.to, block_index),
            OperationResult::EvmTransfered(EvmTransfered(args, tx_hash)) => write!(f, "EvmTransfered: from {} to {} at tx {}", args.account_id, args.to, tx_hash),
            OperationResult::EvmErc20Transfered(EvmErc20Transfered(args, tx_hash)) => write!(f, "EvmErc20Transfered: from {} to {} at tx {}", args.account_id, args.to, tx_hash),
            OperationResult::BtcTransfered(BtcTransfered(args, tx_id)) => write!(f, "BtcTransfered: from {} to {} at tx {}", args.account_id, args.to, tx_id),
            OperationResult::TopUpTransfered(TopUpTransfered(args, block_index)) => write!(f, "TopUpTransfered: from {} to {} at block {}", args.account_id, args.canister_id, block_index),
            OperationResult::CanisterTopUped(CanisterTopUped(args, cycles)) => write!(f, "CanisterTopUped: from {} top up {} cycles for {}", args.account_id, cycles, args.canister_id),
            OperationResult::SignerAdded(_) => write!(f, "SignerAdded"),
            OperationResult::SignerRemoved(_) => write!(f, "SignerRemoved"),
            OperationResult::CanisterUpgraded(_) => write!(f, "CanisterUpgraded"),
            OperationResult::CanisterSettingsUpdated(_) => write!(f, "CanisterSettingsUpdated"),
            OperationResult::AccountCreated(_) => write!(f, "AccountCreated"),
            OperationResult::AccountRemoved(_) => write!(f, "AccountRemoved"),
            OperationResult::AccountRenamed(_) => write!(f, "AccountRenamed"),
            OperationResult::AccountHidden(_) => write!(f, "AccountHidden"),
            OperationResult::AccountUnhidden(_) => write!(f, "AccountUnhidden"),
            OperationResult::EvmRawTransactionSigned(_) => write!(f, "EvmRawTransactionSigned"),
            OperationResult::EvmTransactionSigned(_) => write!(f, "EvmTransactionSigned"),
            OperationResult::EvmContractDeployed(_) => write!(f, "EvmContractDeployed"),
            OperationResult::EvmMessageSigned(_) => write!(f, "EvmMessageSigned"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct TokenSent(pub SendToken, pub SendResult);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct IcpTransfered(pub IcpTransfer, pub TransferBlockIndex);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmTransfered(pub EvmTransfer, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmErc20Transfered(pub EvmTransferErc20, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct TopUpTransfered(pub TopUpTransfer, pub TransferBlockIndex);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct CanisterTopUped(pub NotifyTopUp, pub u128);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct BtcTransfered(pub BtcTransfer, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmContractDeployed {
    pub contract_address: String,
    pub transaction: EvmTransaction1559,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmMessageSigned(pub EvmSignMessage, pub Vec<u8>);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmTransactionSigned(pub EvmSignTranscation, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmRawTransactionSigned(pub EvmSignRawTransaction, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct Empty;

impl ExecutionTrait for Empty {}

use std::fmt;

use super::btc::transfer::BtcTransfer;
use super::evm::sign::{EvmSignMessage, EvmSignRawTransaction, EvmSignTranscation};
use super::evm::transfer::{EvmTransfer, EvmTransferErc20};
use super::icp::transfer::{IcpTransfer, TopUpCanister};
use super::inner::account::{
    CreateAccount, HideAccount, RemoveAccount, RenameAccount, UnhideAccount,
};
use super::inner::setting::{UpdateCanisterSettings, UpgradeCanister};
use super::inner::signer::{AddSigner, RemoveSigner, UpdateSignerThreshold};

use b3_helper_lib::types::{BlockIndex, Cycles};
use b3_wallet_lib::ledger::ckbtc::types::BtcTxId;
use b3_wallet_lib::ledger::evm::london::EvmTransaction1559;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[enum_dispatch]
pub trait ExecutionTrait {}

#[derive(CandidType, Clone, Deserialize, Debug)]
#[enum_dispatch(ExecutionTrait)]
pub enum ExecutionResult {
    IcpTransfered(IcpTransfered),
    EvmTransfered(EvmTransfered),
    EvmErc20Transfered(EvmErc20Transfered),
    CanisterTopUped(CanisterTopUped),
    BtcTransfered(BtcTransfered),
    SignerAdded(AddSigner),
    SignerRemoved(RemoveSigner),
    SignerThresholdUpdated(UpdateSignerThreshold),
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
impl fmt::Display for ExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionResult::IcpTransfered(IcpTransfered(args, block_index)) => {
                write!(f, "IcpTransfered: from {} to {} at block {}", args.account_id, args.to, block_index)
            }
            ExecutionResult::EvmTransfered(EvmTransfered(args, tx_hash)) => {
                write!(f, "EvmTransfered: from {} to {} at tx {}", args.account_id, args.to, tx_hash)
            },
            ExecutionResult::EvmErc20Transfered(EvmErc20Transfered(args, tx_hash)) => {
                write!(f, "EvmErc20Transfered: from {} to {} at tx {}", args.account_id, args.to, tx_hash)
            },
            ExecutionResult::BtcTransfered(BtcTransfered(args, tx_id)) => {
                write!(f, "BtcTransfered: from {} to {} at tx {}", args.account_id, args.to, tx_id)
            },
            ExecutionResult::CanisterTopUped(CanisterTopUped(args, cycles)) => {
                write!(f, "CanisterTopUped: from {} top up {} cycles for {}", args.account_id, cycles, args.canister_id)
            },
            ExecutionResult::SignerAdded(_) => write!(f, "SignerAdded"),
            ExecutionResult::SignerRemoved(_) => write!(f, "SignerRemoved"),
            ExecutionResult::SignerThresholdUpdated(_) => write!(f, "SignerThresholdUpdated"),
            ExecutionResult::CanisterUpgraded(_) => write!(f, "CanisterUpgraded"),
            ExecutionResult::CanisterSettingsUpdated(_) => write!(f, "CanisterSettingsUpdated"),
            ExecutionResult::AccountCreated(_) => write!(f, "AccountCreated"),
            ExecutionResult::AccountRemoved(_) => write!(f, "AccountRemoved"),
            ExecutionResult::AccountRenamed(_) => write!(f, "AccountRenamed"),
            ExecutionResult::AccountHidden(_) => write!(f, "AccountHidden"),
            ExecutionResult::AccountUnhidden(_) => write!(f, "AccountUnhidden"),
            ExecutionResult::EvmRawTransactionSigned(_) => write!(f, "EvmRawTransactionSigned"),
            ExecutionResult::EvmTransactionSigned(_) => write!(f, "EvmTransactionSigned"),
            ExecutionResult::EvmContractDeployed(_) => write!(f, "EvmContractDeployed"),
            ExecutionResult::EvmMessageSigned(_) => write!(f, "EvmMessageSigned"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct IcpTransfered(pub IcpTransfer, pub BlockIndex);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmTransfered(pub EvmTransfer, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct EvmErc20Transfered(pub EvmTransferErc20, pub String);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct CanisterTopUped(pub TopUpCanister, pub Cycles);

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct BtcTransfered(pub BtcTransfer, pub BtcTxId);

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

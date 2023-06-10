use super::inner::account::{
    CreateAccount, HideAccount, RemoveAccount, RenameAccount, UnhideAccount,
};
use super::inner::setting::{UpdateCanisterSettings, UpgradeCanister};
use super::inner::signer::{AddSigner, RemoveSigner, UpdateSignerThreshold};

use b3_helper_lib::types::{BlockIndex, Cycles};
use b3_wallet_lib::ledger::evm::evm::EvmTransaction;
use b3_wallet_lib::ledger::evm::london::EvmTransaction1559;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[enum_dispatch]
pub trait ExecutionTrait {}

#[derive(CandidType, Clone, Deserialize)]
#[enum_dispatch(ExecutionTrait)]
pub enum ExecutionResult {
    IcpTransfered(IcpTransfered),
    EvmTransfered(EvmTransfered),
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
    EvmContractDeployed(EvmContractDeployed),
    EvmMessageSigned(EvmMessageSigned),
}

#[derive(CandidType, Clone, Deserialize)]
pub struct IcpTransfered(pub BlockIndex);

#[derive(CandidType, Clone, Deserialize)]
pub struct CanisterTopUped(pub Cycles);

#[derive(CandidType, Clone, Deserialize)]
pub struct BtcTransfered(pub String);

#[derive(CandidType, Clone, Deserialize)]
pub struct EvmTransfered(pub EvmTransaction1559);

#[derive(CandidType, Clone, Deserialize)]
pub struct EvmContractDeployed {
    pub contract_address: String,
    pub transaction: EvmTransaction1559,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct EvmMessageSigned(pub Vec<u8>);

#[derive(CandidType, Clone, Deserialize)]
pub struct EvmTransactionSigned(pub EvmTransaction);

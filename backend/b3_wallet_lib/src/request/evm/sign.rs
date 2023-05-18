use super::EvmRequest;
use crate::{
    error::WalletError,
    evm::{get_evm_transaction, EvmTransaction},
    request::Request,
    store::with_ledger,
    types::SignedMessage,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// EVM TRANSACTION
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignTranscationRequest {
    pub account_id: String,
    pub chain_id: u64,
    pub message: Vec<u8>,
    pub transaction: EvmTransaction,
}

impl From<EvmSignTranscationRequest> for Request {
    fn from(args: EvmSignTranscationRequest) -> Self {
        EvmRequest::EvmSignTranscationRequest(args).into()
    }
}

impl EvmSignTranscationRequest {
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let signed = ledger.sign_with_ecdsa(self.message.clone()).await?;

        Ok(signed)
    }
}

// EVM SIGN TRANSACTION MESSAGE
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignRawTransactionRequest {
    pub account_id: String,
    pub hex_raw_tx: Vec<u8>,
    pub chain_id: u64,
}

impl TryFrom<EvmSignRawTransactionRequest> for EvmSignTranscationRequest {
    type Error = WalletError;

    fn try_from(args: EvmSignRawTransactionRequest) -> Result<Self, WalletError> {
        let tx = get_evm_transaction(&args.hex_raw_tx, args.chain_id)?;

        let message = tx.get_message_to_sign()?;

        let transaction = tx.get_transaction();

        Ok(EvmSignTranscationRequest {
            account_id: args.account_id,
            message,
            chain_id: args.chain_id,
            transaction,
        })
    }
}

impl From<EvmSignRawTransactionRequest> for Request {
    fn from(args: EvmSignRawTransactionRequest) -> Self {
        EvmRequest::EvmSignRawTransactionRequest(args).into()
    }
}

impl EvmSignRawTransactionRequest {
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let signed = ledger.sign_with_ecdsa(self.hex_raw_tx.clone()).await?;

        Ok(signed)
    }
}

// EVM SIGN MESSAGE
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignMessageRequest {
    pub account_id: String,
    pub message: Vec<u8>,
}

impl From<EvmSignMessageRequest> for Request {
    fn from(args: EvmSignMessageRequest) -> Self {
        EvmRequest::EvmSignMessageRequest(args).into()
    }
}

impl EvmSignMessageRequest {
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let signed = ledger.sign_with_ecdsa(self.message.clone()).await?;

        Ok(signed)
    }
}

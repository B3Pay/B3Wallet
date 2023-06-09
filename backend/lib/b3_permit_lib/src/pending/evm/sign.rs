use crate::{error::RequestError, pending::RequestTrait, types::ConsentMessageResponse};
use async_trait::async_trait;
use b3_wallet_lib::{
    error::WalletError,
    ledger::evm::api::{get_evm_transaction, EvmTransaction},
    store::with_ledger,
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

#[async_trait]
impl RequestTrait for EvmSignTranscationRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let _signed = ledger.sign_with_ecdsa(self.message.clone()).await?;

        todo!("return signed tx")
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        // check if the chain id is initialized
        with_ledger(&self.account_id, |ledger| {
            if ledger.evm(self.chain_id).is_some() {
                Ok(())
            } else {
                Err(RequestError::ChainIdNotInitialized)
            }
        })?
    }

    fn method_name(&self) -> String {
        "evm_sign_transaction".to_string()
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

#[async_trait]
impl RequestTrait for EvmSignRawTransactionRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let _signed = ledger.sign_with_ecdsa(self.hex_raw_tx.clone()).await?;

        todo!("return signed tx")
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        // check if the chain id is initialized
        with_ledger(&self.account_id, |ledger| {
            if ledger.evm(self.chain_id).is_some() {
                Ok(())
            } else {
                Err(RequestError::ChainIdNotInitialized)
            }
        })?
    }

    fn method_name(&self) -> String {
        "evm_sign_transaction".to_string()
    }
}

// EVM SIGN MESSAGE
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignMessageRequest {
    pub account_id: String,
    pub message: Vec<u8>,
}

#[async_trait]
impl RequestTrait for EvmSignMessageRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let _signed = ledger.sign_with_ecdsa(self.message.clone()).await?;

        todo!("return signed tx")
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        // check if the chain id is initialized
        with_ledger(&self.account_id, |ledger| {
            if ledger.evm(1).is_some() {
                Ok(())
            } else {
                Err(RequestError::ChainIdNotInitialized)
            }
        })?
    }

    fn method_name(&self) -> String {
        "evm_sign_message".to_string()
    }
}

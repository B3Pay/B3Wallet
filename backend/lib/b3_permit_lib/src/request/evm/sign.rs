use crate::{
    error::PermitError,
    request::{
        request::RequestTrait,
        result::{
            EvmMessageSigned, EvmRawTransactionSigned, EvmTransactionSigned, ExecutionResult,
        },
    },
};
use async_trait::async_trait;
use b3_wallet_lib::{
    error::WalletError,
    ledger::evm::evm::{get_evm_transaction, EvmSignTrait, EvmTransaction},
    store::with_ledger,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// EVM TRANSACTION
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignTranscation {
    pub account_id: String,
    pub chain_id: u64,
    pub transaction: EvmTransaction,
}

#[async_trait]
impl RequestTrait for EvmSignTranscation {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;
        let public_key = ledger.eth_public_key()?;

        let mut transaction = self.transaction.clone();

        transaction.unsigned_serialized();

        let signature = ledger.sign_with_ecdsa(transaction.serialized()).await?;

        transaction.sign(signature, public_key)?;

        Ok(EvmTransactionSigned(self, transaction.tx_id()).into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        // check if the chain id is initialized
        with_ledger(&self.account_id, |ledger| {
            if ledger.evm(self.chain_id).is_some() {
                Ok(())
            } else {
                Err(PermitError::ChainIdNotInitialized)
            }
        })?
    }

    fn method_name(&self) -> String {
        "evm_sign_transaction".to_string()
    }
}

// EVM SIGN TRANSACTION MESSAGE
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignRawTransaction {
    pub account_id: String,
    pub hex_raw_tx: Vec<u8>,
    pub chain_id: u64,
}

impl TryFrom<EvmSignRawTransaction> for EvmSignTranscation {
    type Error = PermitError;

    fn try_from(args: EvmSignRawTransaction) -> Result<Self, PermitError> {
        let transaction = get_evm_transaction(&args.hex_raw_tx, args.chain_id)?;

        Ok(EvmSignTranscation {
            account_id: args.account_id,
            chain_id: args.chain_id,
            transaction,
        })
    }
}

#[async_trait]
impl RequestTrait for EvmSignRawTransaction {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let public_key = ledger.eth_public_key()?;

        let mut transaction = get_evm_transaction(&self.hex_raw_tx, self.chain_id)?;

        transaction.unsigned_serialized();

        let signature = ledger.sign_with_ecdsa(transaction.serialized()).await?;

        transaction.sign(signature, public_key)?;

        Ok(EvmRawTransactionSigned(self, transaction.tx_id()).into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        // check if the chain id is initialized
        with_ledger(&self.account_id, |ledger| {
            if ledger.evm(self.chain_id).is_some() {
                Ok(())
            } else {
                Err(PermitError::ChainIdNotInitialized)
            }
        })??;

        // check if the hex_raw_tx is valid
        let transaction = get_evm_transaction(&self.hex_raw_tx, self.chain_id)
            .map_err(|_| PermitError::InvalidTransaction)?;

        let chain_id = transaction.chain_id();

        // check if the transaction is valid
        if chain_id != self.chain_id {
            return Err(PermitError::InvalidChainId(chain_id, self.chain_id));
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "evm_sign_transaction".to_string()
    }
}

// EVM SIGN MESSAGE
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmSignMessage {
    pub account_id: String,
    pub chain_id: u64,
    pub message: Vec<u8>,
}

#[async_trait]
impl RequestTrait for EvmSignMessage {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let signed = ledger.sign_with_ecdsa(self.message.clone()).await?;

        Ok(EvmMessageSigned(self, signed).into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        // check if the chain id is initialized
        with_ledger(&self.account_id, |ledger| {
            if ledger.evm(1).is_some() {
                Ok(())
            } else {
                Err(PermitError::ChainIdNotInitialized)
            }
        })??;

        // check if the message is not sneaky transaction
        let transaction = get_evm_transaction(&self.message, self.chain_id);

        if transaction.is_ok() {
            return Err(PermitError::SneakyMessage);
        }

        Ok(())
    }

    fn method_name(&self) -> String {
        "evm_sign_message".to_string()
    }
}

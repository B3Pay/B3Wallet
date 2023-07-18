use crate::{
    error::PermitError,
    request::result::{EvmTransfered, ExecutionResult},
    request::{request::RequestTrait, result::EvmErc20Transfered},
};

use async_trait::async_trait;
use b3_wallet_lib::{
    error::WalletError,
    ledger::{
        evm::{evm::EvmSignTrait, london::EvmTransaction1559, utils::get_transfer_data},
        subaccount::SubaccountTrait,
        types::ChainEnum,
    },
    store::{with_chain, with_ledger},
};
use candid::{CandidType, Deserialize};

// TRANSFER ETH
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmTransfer {
    pub account_id: String,
    pub chain_id: u64,
    pub nonce: u64,
    pub to: String,
    pub value: u64,
    pub gas_limit: Option<u64>,
    pub max_fee_per_gas: Option<u64>,
    pub max_priority_fee_per_gas: Option<u64>,
}

#[async_trait]
impl RequestTrait for EvmTransfer {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let public_key = ledger.eth_public_key()?;

        // TODO: get default gas limit from user settings
        let gas_limit = self.gas_limit.unwrap_or(0);
        let max_fee_per_gas = self.max_fee_per_gas.unwrap_or(0);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas.unwrap_or(0);

        let mut transaction = EvmTransaction1559 {
            nonce: self.nonce,
            chain_id: self.chain_id,
            to: self.to.clone(),
            value: self.value,
            data: "0x00".to_string(),
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            access_list: vec![],
            v: "0x00".to_string(),
            r: "0x00".to_string(),
            s: "0x00".to_string(),
        };

        let raw_tx = transaction.unsigned_serialized();

        let _signed = ledger.subaccount.sign_with_ecdsa(raw_tx).await?;

        transaction.sign(_signed, public_key)?;

        Ok(EvmTransfered(self, transaction.tx_id()).into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        // check if the chain id is initialized
        with_chain(&self.account_id, &ChainEnum::EVM(self.chain_id), |_| Ok(()))?
    }

    fn method_name(&self) -> String {
        "evm_transfer_eth".to_string()
    }

    fn title(&self) -> String {
        format!("Transfer {} ETH", self.value)
    }

    fn message(&self) -> String {
        format!("Transfer {} ETH", self.value)
    }
}

// EVM TRANSFER ERC20
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmTransferErc20 {
    pub account_id: String,
    pub chain_id: u64,
    pub nonce: u64,
    pub to: String,
    pub value: u64,
    pub contract_address: String,
    pub gas_limit: Option<u64>,
    pub max_fee_per_gas: Option<u64>,
    pub max_priority_fee_per_gas: Option<u64>,
}

#[async_trait]
impl RequestTrait for EvmTransferErc20 {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let public_key = ledger.eth_public_key()?;

        let data = "0x".to_owned() + &get_transfer_data(&self.to, self.value)?;

        // TODO: get default gas limit from user settings
        let gas_limit = self.gas_limit.unwrap_or(0);
        let max_fee_per_gas = self.max_fee_per_gas.unwrap_or(0);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas.unwrap_or(0);

        let mut transaction = EvmTransaction1559 {
            nonce: self.nonce,
            chain_id: self.chain_id,
            max_priority_fee_per_gas,
            gas_limit,
            max_fee_per_gas,
            to: self.contract_address.clone(),
            value: 0,
            data,
            access_list: vec![],
            v: "0x00".to_string(),
            r: "0x00".to_string(),
            s: "0x00".to_string(),
        };

        let raw_tx = transaction.unsigned_serialized();

        let signature = ledger.subaccount.sign_with_ecdsa(raw_tx).await?;

        transaction.sign(signature, public_key)?;

        Ok(EvmErc20Transfered(self, transaction.tx_id()).into())
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        // check if the chain id is initialized
        with_chain(&self.account_id, &ChainEnum::EVM(self.chain_id), |_| Ok(()))?
    }

    fn method_name(&self) -> String {
        self.contract_address.clone()
    }

    fn title(&self) -> String {
        format!("Transfer {} ERC20", self.value)
    }

    fn message(&self) -> String {
        format!("Transfer {} ERC20", self.value)
    }
}

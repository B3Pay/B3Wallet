use crate::{pending::Request, types::ConsentMessageResponse};

use super::EvmRequest;

use b3_wallet_lib::{
    error::WalletError,
    ledger::evm::{sign::EvmSign, tx1559::EvmTransaction1559, utils::get_transfer_data},
    store::with_ledger,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// TRANSFER ETH
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmTransferEthRequest {
    account_id: String,
    chain_id: u64,
    nonce: u64,
    to: String,
    value: u64,
    gas_limit: Option<u64>,
    max_fee_per_gas: Option<u64>,
    max_priority_fee_per_gas: Option<u64>,
}

impl From<EvmTransferEthRequest> for Request {
    fn from(args: EvmTransferEthRequest) -> Self {
        EvmRequest::EvmTransferEthRequest(args).into()
    }
}

impl EvmTransferEthRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        // TODO: get default gas limit from user settings
        let gas_limit = self.gas_limit.unwrap_or(0);
        let max_fee_per_gas = self.max_fee_per_gas.unwrap_or(0);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas.unwrap_or(0);

        let tx = EvmTransaction1559 {
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

        let raw_tx = tx.get_message_to_sign()?;

        let _signed = ledger.sign_with_ecdsa(raw_tx).await?;

        Ok(ConsentMessageResponse::default())
    }
}

// EVM TRANSFER ERC20
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmTransferErc20Request {
    account_id: String,
    chain_id: u64,
    nonce: u64,
    address: String,
    value: u64,
    contract_address: String,
    gas_limit: Option<u64>,
    max_fee_per_gas: Option<u64>,
    max_priority_fee_per_gas: Option<u64>,
}

impl From<EvmTransferErc20Request> for Request {
    fn from(args: EvmTransferErc20Request) -> Self {
        EvmRequest::EvmTransferErc20Request(args).into()
    }
}

impl EvmTransferErc20Request {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let data = "0x".to_owned() + &get_transfer_data(&self.address, self.value)?;

        // TODO: get default gas limit from user settings
        let gas_limit = self.gas_limit.unwrap_or(0);
        let max_fee_per_gas = self.max_fee_per_gas.unwrap_or(0);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas.unwrap_or(0);

        let tx = EvmTransaction1559 {
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

        let raw_tx = tx.serialize()?;

        let _signed = ledger.sign_with_ecdsa(raw_tx).await?;

        Ok(ConsentMessageResponse::default())
    }
}

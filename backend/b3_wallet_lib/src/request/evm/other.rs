use super::EvmRequest;
use crate::{
    error::WalletError,
    evm::{tx1559::EvmTransaction1559, EvmSign},
    request::Request,
    store::with_ledger,
    types::SignedMessage,
    utils::vec_u8_to_string,
};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

// DEPLOY CONTRACT
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmDeployContractRequest {
    account_id: String,
    chain_id: u64,
    nonce: u64,
    hex_byte_code: Vec<u8>,
    gas_limit: Option<u64>,
    max_fee_per_gas: Option<u64>,
    max_priority_fee_per_gas: Option<u64>,
}

impl From<EvmDeployContractRequest> for Request {
    fn from(args: EvmDeployContractRequest) -> Self {
        EvmRequest::EvmDeployContractRequest(args).into()
    }
}

impl EvmDeployContractRequest {
    pub async fn execute(&self) -> Result<SignedMessage, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let data = "0x".to_owned() + &vec_u8_to_string(&self.hex_byte_code);

        // TODO: get default gas limit from user settings
        let gas_limit = self.gas_limit.unwrap_or(0);
        let max_fee_per_gas = self.max_fee_per_gas.unwrap_or(0);
        let max_priority_fee_per_gas = self.max_priority_fee_per_gas.unwrap_or(0);

        let tx = EvmTransaction1559 {
            nonce: self.nonce,
            chain_id: self.chain_id,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            to: "0x".to_string(),
            value: 0,
            data,
            access_list: vec![],
            v: "0x00".to_string(),
            r: "0x00".to_string(),
            s: "0x00".to_string(),
        };

        let raw_tx = tx.serialize()?;

        let signed = ledger.sign_with_ecdsa(raw_tx).await?;

        Ok(signed)
    }
}

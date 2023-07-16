use crate::{
    error::PermitError,
    request::result::ExecutionResult,
    request::{request::RequestTrait, result::BtcTransfered},
};
use async_trait::async_trait;
use b3_helper_lib::amount::TokenAmount;
use b3_wallet_lib::ledger::types::ChainEnum;
use b3_wallet_lib::ledger::{chain::ChainTrait, types::SendResult};
use b3_wallet_lib::{error::WalletError, ledger::btc::network::BtcNetwork, store::with_chain};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct BtcTransfer {
    pub account_id: String,
    pub amount: TokenAmount,
    pub to: String,
    pub network: BtcNetwork,
}

#[async_trait]
impl RequestTrait for BtcTransfer {
    async fn execute(self) -> Result<ExecutionResult, WalletError> {
        let chain = with_chain(&self.account_id, &ChainEnum::BTC(self.network), |chain| {
            chain.clone()
        })?;

        let result = chain.send(self.to.clone(), self.amount).await;

        match result {
            Ok(SendResult::BTC(txid)) => Ok(BtcTransfered(self, txid).into()),
            Err(err) => return Err(WalletError::ExecutionError(err.to_string())),
            _ => return Err(WalletError::UnknownError),
        }
    }

    fn validate_request(&self) -> Result<(), PermitError> {
        if self.amount <= TokenAmount::from(0) {
            return Err(PermitError::InvalidAmount);
        }

        with_chain(&self.account_id, &ChainEnum::BTC(self.network), |_| Ok(()))?
    }

    fn method_name(&self) -> String {
        "btc_transfer".to_string()
    }

    fn title(&self) -> String {
        format!("Send {} {}", self.amount, self.network)
    }

    fn message(&self) -> String {
        format!("Send {} {}", self.amount, self.network)
    }
}

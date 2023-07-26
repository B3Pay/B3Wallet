use crate::{
    error::OperationError,
    operation::result::OperationResult,
    operation::{result::BtcTransfered, OperationTrait},
};
use async_trait::async_trait;
use b3_utils::currency::TokenAmount;
use b3_wallet_lib::ledger::types::ChainEnum;
use b3_wallet_lib::ledger::{chain::ChainTrait, types::SendResult};
use b3_wallet_lib::{error::WalletError, ledger::btc::network::BtcNetwork, store::with_chain};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct BtcTransfer {
    pub account_id: String,
    pub amount: TokenAmount,
    pub to: String,
    pub network: BtcNetwork,
}

#[async_trait]
impl OperationTrait for BtcTransfer {
    async fn execute(self) -> Result<OperationResult, WalletError> {
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

    fn validate_request(&self) -> Result<(), OperationError> {
        if self.amount <= TokenAmount::from(0) {
            return Err(OperationError::InvalidAmount);
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

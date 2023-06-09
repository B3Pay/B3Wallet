use crate::{
    error::RequestError,
    pending::RequestTrait,
    types::{ConsentInfo, ConsentMessageResponse},
};
use async_trait::async_trait;
use b3_wallet_lib::{error::WalletError, ledger::btc::network::BtcNetwork, store::with_ledger};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct BtcTransferRequest {
    pub account_id: String,
    pub amount: u64,
    pub to: String,
    pub network: BtcNetwork,
}

#[async_trait]
impl RequestTrait for BtcTransferRequest {
    async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let result = ledger
            .bitcoin_transfer(self.network, &self.to, self.amount)
            .await;

        match result {
            Err(err) => return Err(WalletError::ExecutionError(err.to_string())),
            Ok(tx_id) => Ok(ConsentMessageResponse::Valid(ConsentInfo::new(format!(
                "Transfer {} BTC to {} on {}, tx_id: {}",
                self.amount,
                self.to,
                self.network.to_string(),
                tx_id
            )))),
        }
    }

    fn validate_request(&self) -> Result<(), RequestError> {
        if self.amount == 0 {
            return Err(RequestError::InvalidAmount);
        }

        with_ledger(&self.account_id, |ledger| {
            if ledger.btc(self.network).is_some() {
                Ok(())
            } else {
                Err(RequestError::ChainIdNotInitialized)
            }
        })?
    }

    fn method_name(&self) -> String {
        "btc_transfer".to_string()
    }
}

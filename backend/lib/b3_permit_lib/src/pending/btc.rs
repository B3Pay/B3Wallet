use std::fmt;

use b3_helper_lib::error::TrapError;
use b3_wallet_lib::{error::WalletError, ledger::btc::network::BtcNetwork, store::with_ledger};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::types::{ConsendInfo, ConsentMessageResponse};

use super::Request;

#[enum_dispatch]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum BtcRequest {
    BtcTransferRequest,
}

impl BtcRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        match self {
            BtcRequest::BtcTransferRequest(args) => args.execute().await,
        }
    }
}

impl fmt::Display for BtcRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BtcRequest::BtcTransferRequest(_) => write!(f, "BtcTransferRequest"),
        }
    }
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct BtcTransferRequest {
    pub account_id: String,
    pub amount: u64,
    pub to: String,
    pub network: BtcNetwork,
}

impl From<BtcTransferRequest> for Request {
    fn from(args: BtcTransferRequest) -> Self {
        Request::BtcRequest(BtcRequest::BtcTransferRequest(args))
    }
}

impl BtcTransferRequest {
    pub async fn execute(&self) -> Result<ConsentMessageResponse, WalletError> {
        let ledger = with_ledger(&self.account_id, |ledger| ledger.clone())?;

        let result = ledger
            .bitcoin_transfer(self.network, &self.to, self.amount)
            .await;

        match result {
            Err(err) => return Err(WalletError::BitcoinSendTransactionError(err.to_string())),
            Ok(tx_id) => Ok(ConsentMessageResponse::Valid(ConsendInfo {
                consent_message: format!(
                    "Transfer {} BTC to {} on {}, tx_id: {}",
                    self.amount,
                    self.to,
                    self.network.to_string(),
                    tx_id
                ),
                ..Default::default()
            })),
        }
    }
}

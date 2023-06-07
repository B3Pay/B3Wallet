use crate::ledger::{
    btc::network::BtcNetwork,
    icrc::{icrc1::ICRC1, types::ICRC1TransferArgs},
    types::Ledger,
};
use b3_helper_lib::{constants::CKBTC_LEDGER_CANISTER, error::ErrorTrait};

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

use super::{
    minter::Minter,
    types::{BtcTxId, RetrieveBtcOk, RetrieveBtcResult, Satoshi, UpdateBalanceResult},
};

impl Ledger {
    pub async fn update_balance(&self) -> Result<UpdateBalanceResult, CkbtcError> {
        let owner = ic_cdk_id();
        let subaccount = self.subaccount.clone();

        let result = Minter::update_balance(Some(owner), Some(subaccount))
            .await
            .map_err(|err| CkbtcError::UpdateCkbtcBalance(err.to_string()))?;

        Ok(result)
    }

    pub async fn swap_btc_to_ckbtc(
        &self,
        btc_network: BtcNetwork,
        amount: Satoshi,
    ) -> Result<BtcTxId, CkbtcError> {
        let owner = ic_cdk_id();
        let subaccount = self.subaccount.clone();

        let dst_address = Minter::get_btc_address(Some(owner), Some(subaccount))
            .await
            .map_err(|err| CkbtcError::SwapBtcToCkbtc(err.to_string()))?;

        let tx_id = self
            .bitcoin_transfer(btc_network, &dst_address, amount)
            .await
            .map_err(|err| CkbtcError::SwapBtcToCkbtc(err.to_string()))?;

        Ok(tx_id)
    }

    pub async fn swap_ckbtc_to_btc(
        &self,
        btc_address: String,
        amount: Satoshi,
    ) -> Result<RetrieveBtcOk, CkbtcError> {
        let withdraw_account = Minter::get_withdrawal_account()
            .await
            .map_err(|err| CkbtcError::SwapCkbtcToBtc(err.to_string()))?;

        let ckbtc = ICRC1::new(CKBTC_LEDGER_CANISTER);

        let args = ICRC1TransferArgs {
            to: withdraw_account,
            amount: amount.into(),
            memo: None,
            fee: None,
            created_at_time: None,
            from_subaccount: Some(self.subaccount.clone()),
        };

        let result = ckbtc
            .transfer(args)
            .await
            .map_err(|err| CkbtcError::SwapCkbtcToBtc(err.to_string()))?;

        match result {
            Ok(_) => {
                let block_index = Minter::retrieve_btc(btc_address, amount)
                    .await
                    .map_err(|err| CkbtcError::SwapCkbtcToBtc(err.to_string()))?;

                match block_index {
                    RetrieveBtcResult::Ok(block_index) => Ok(block_index),
                    RetrieveBtcResult::Err(err) => Err(CkbtcError::SwapCkbtcToBtc(err.to_string())),
                }
            }
            Err(err) => Err(CkbtcError::SwapCkbtcToBtc(err.to_string())),
        }
    }
}

pub enum CkbtcError {
    UpdateCkbtcBalance(String),
    SwapBtcToCkbtc(String),
    SwapCkbtcToBtc(String),
}

#[rustfmt::skip]
impl ErrorTrait for CkbtcError {
    fn to_string(self) -> String {
        match self {
            CkbtcError::SwapBtcToCkbtc(msg) => format!("Swap BTC to CKBTC failed: {}", msg.to_string()),
            CkbtcError::SwapCkbtcToBtc(msg) => format!("Swap CKBTC to BTC failed: {}", msg),
            CkbtcError::UpdateCkbtcBalance(msg) => format!("Update CKBTC balance failed: {}", msg),
        }
    }
}

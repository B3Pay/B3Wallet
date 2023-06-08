use super::minter::Minter;
use super::types::{RetrieveBtcOk, RetrieveBtcResult, Satoshi, UpdateBalanceResult};
use crate::{
    error::WalletError,
    ledger::{
        btc::network::BtcNetwork,
        icrc::{
            icrc1::ICRC1,
            types::{ICRC1TransferArgs, ICRCMemo, ICRCTimestamp, ICRCTokens},
        },
    },
};
use b3_helper_lib::{
    account::ICRCAccount,
    constants::{CKBTC_LEDGER_CANISTER_MAINNET, CKBTC_LEDGER_CANISTER_TESTNET},
    error::ErrorTrait,
    subaccount::Subaccount,
};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct CKBTC {
    pub ledger: ICRC1,
    pub minter: Minter,
    pub account: ICRCAccount,
    pub fee: Option<ICRCTokens>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
    pub pending: Option<String>,
}

impl CKBTC {
    pub async fn new(btc_network: BtcNetwork, subaccount: Subaccount) -> Result<Self, WalletError> {
        let ledger = match btc_network {
            BtcNetwork::Testnet => ICRC1(CKBTC_LEDGER_CANISTER_TESTNET),
            BtcNetwork::Mainnet => ICRC1(CKBTC_LEDGER_CANISTER_MAINNET),
            BtcNetwork::Regtest => ICRC1(CKBTC_LEDGER_CANISTER_MAINNET),
        };

        let minter = Minter(btc_network);

        let fee = ledger
            .fee()
            .await
            .map_err(|e| WalletError::ICRC1CallError(e.to_string()))?;

        let owner = ic_cdk_id();
        let account = ICRCAccount::new(owner, Some(subaccount));

        Ok(CKBTC {
            ledger,
            minter,
            account,
            fee: Some(fee),
            memo: None,
            created_at_time: None,
            pending: None,
        })
    }
}

impl CKBTC {
    pub fn add_pending(&mut self, txid: String) {
        self.pending = Some(txid);
    }

    pub async fn get_btc_address(&self) -> Result<String, WalletError> {
        let account = self.account.clone();

        self.minter.get_btc_address(account).await
    }

    pub async fn update_balance(&self) -> Result<UpdateBalanceResult, WalletError> {
        let account = self.account.clone();

        let result = self
            .minter
            .update_balance(account)
            .await
            .map_err(|err| WalletError::CkbtcUpdateBalance(err.to_string()))?;

        Ok(result)
    }

    pub async fn swap_ckbtc_to_btc(
        &self,
        retrieve_address: String,
        amount: Satoshi,
    ) -> Result<RetrieveBtcOk, WalletError> {
        let withdraw_account = self
            .minter
            .get_withdrawal_account()
            .await
            .map_err(|err| WalletError::CkbtcSwapToBtcError(err.to_string()))?;

        let args = ICRC1TransferArgs {
            to: withdraw_account,
            amount: amount.into(),
            memo: None,
            fee: None,
            created_at_time: None,
            from_subaccount: self.account.subaccount(),
        };

        let result = self
            .ledger
            .transfer(args)
            .await
            .map_err(|err| WalletError::CkbtcSwapToBtcError(err.to_string()))?;

        match result {
            Ok(_) => {
                let block_index = self
                    .minter
                    .retrieve_btc(retrieve_address, amount)
                    .await
                    .map_err(|err| WalletError::CkbtcSwapToBtcError(err.to_string()))?;

                match block_index {
                    RetrieveBtcResult::Ok(block_index) => Ok(block_index),
                    RetrieveBtcResult::Err(err) => {
                        Err(WalletError::CkbtcSwapToBtcError(err.to_string()))
                    }
                }
            }
            Err(err) => Err(WalletError::CkbtcSwapToBtcError(err.to_string())),
        }
    }
}

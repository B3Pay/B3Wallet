use super::error::CkbtcError;
use super::minter::Minter;
use super::types::{BtcTxId, RetrieveBtcOk, RetrieveBtcResult, Satoshi, UpdateBalanceResult};
use crate::ledger::types::Pendings;
use crate::ledger::{
    btc::network::BtcNetwork,
    icrc::{
        icrc1::ICRC1,
        types::{ICRC1TransferArgs, ICRCMemo, ICRCTimestamp, ICRCTokens},
    },
};
use b3_helper_lib::{
    account::ICRCAccount,
    constants::{CKBTC_LEDGER_CANISTER_MAINNET, CKBTC_LEDGER_CANISTER_TESTNET},
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
pub struct CkbtcChain {
    pub ledger: ICRC1,
    pub minter: Minter,
    pub account: ICRCAccount,
    pub fee: Option<ICRCTokens>,
    pub memo: Option<ICRCMemo>,
    pub pending_receive: Pendings,
    pub created_at_time: Option<ICRCTimestamp>,
}

impl CkbtcChain {
    pub fn has_pending(&self, tx_id: &BtcTxId) -> bool {
        self.pending_receive.contains(tx_id)
    }

    pub fn add_pending(&mut self, tx_id: BtcTxId) {
        self.pending_receive.push(tx_id);
    }

    pub fn remove_pending(&mut self, tx_id: &BtcTxId) {
        self.pending_receive.retain(|x| x != tx_id);
    }

    pub fn clear_pending(&mut self) {
        self.pending_receive.clear();
    }
}

impl CkbtcChain {
    pub async fn new(btc_network: BtcNetwork, subaccount: Subaccount) -> Result<Self, CkbtcError> {
        let ledger = match btc_network {
            BtcNetwork::Testnet => ICRC1(CKBTC_LEDGER_CANISTER_TESTNET),
            BtcNetwork::Mainnet => ICRC1(CKBTC_LEDGER_CANISTER_MAINNET),
            BtcNetwork::Regtest => ICRC1(CKBTC_LEDGER_CANISTER_MAINNET),
        };

        let minter = Minter(btc_network);

        let fee = ledger.fee().await.map_err(CkbtcError::IcrcError)?;

        let owner = ic_cdk_id();
        let account = ICRCAccount::new(owner, Some(subaccount));

        Ok(CkbtcChain {
            ledger,
            minter,
            account,
            memo: None,
            fee: Some(fee),
            created_at_time: None,
            pending_receive: vec![],
        })
    }

    pub async fn get_btc_address(&self) -> Result<String, CkbtcError> {
        let account = self.account.clone();

        self.minter
            .get_btc_address(account)
            .await
            .map_err(CkbtcError::MinterError)
    }

    pub async fn update_balance(&self) -> Result<UpdateBalanceResult, CkbtcError> {
        let account = self.account.clone();

        let result = self
            .minter
            .update_balance(account)
            .await
            .map_err(CkbtcError::MinterError)?;

        Ok(result)
    }

    pub async fn swap_to_btc(
        &self,
        retrieve_address: String,
        amount: Satoshi,
    ) -> Result<RetrieveBtcOk, CkbtcError> {
        let withdraw_account = self
            .minter
            .get_withdrawal_account()
            .await
            .map_err(CkbtcError::MinterError)?;

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
            .map_err(|err| CkbtcError::CkbtcSwapToBtcError(err.to_string()))?;

        match result {
            Ok(_) => {
                let block_index = self
                    .minter
                    .retrieve_btc(retrieve_address, amount)
                    .await
                    .map_err(CkbtcError::MinterError)?;

                match block_index {
                    RetrieveBtcResult::Ok(block_index) => Ok(block_index),
                    RetrieveBtcResult::Err(err) => {
                        Err(CkbtcError::CkbtcSwapToBtcError(err.to_string()))
                    }
                }
            }
            Err(err) => Err(CkbtcError::CkbtcSwapToBtcError(err.to_string())),
        }
    }
}

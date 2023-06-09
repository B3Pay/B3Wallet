use crate::ledger::btc::network::BtcNetwork;

use super::{
    error::MinterError,
    types::{
        GetBtcAddressArgs, RetrieveBtcArgs, RetrieveBtcResult, RetrieveBtcStatus,
        RetrieveBtcStatusRequest, UpdateBalanceArgs, UpdateBalanceResult,
    },
};
use b3_helper_lib::{
    account::ICRCAccount,
    constants::CKBTC_MINTER_CANISTER_MAINNET,
    types::{BlockIndex, CanisterId},
};
use ic_cdk::api::call::call;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Minter(pub BtcNetwork);

impl Minter {
    pub fn canister_id(&self) -> CanisterId {
        match self.0 {
            BtcNetwork::Testnet => CanisterId::from_text("ml52i-qqaaa-aaaar-qaaba-cai").unwrap(),
            BtcNetwork::Regtest => CKBTC_MINTER_CANISTER_MAINNET,
            BtcNetwork::Mainnet => CKBTC_MINTER_CANISTER_MAINNET,
        }
    }

    pub async fn get_btc_address(&self, account: ICRCAccount) -> Result<String, MinterError> {
        let args = GetBtcAddressArgs {
            owner: Some(account.owner()),
            subaccount: account.subaccount(),
        };

        let (btc_address,): (String,) = call(self.canister_id(), "get_btc_address", (args,))
            .await
            .map_err(|err| MinterError::CallError(err.1))?;

        Ok(btc_address)
    }

    pub async fn get_withdrawal_account(&self) -> Result<ICRCAccount, MinterError> {
        let (withdrawal_account,): (ICRCAccount,) =
            call(self.canister_id(), "get_withdrawal_account", ())
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(withdrawal_account)
    }

    pub async fn update_balance(
        &self,
        account: ICRCAccount,
    ) -> Result<UpdateBalanceResult, MinterError> {
        let args = UpdateBalanceArgs {
            owner: Some(account.owner()),
            subaccount: account.subaccount(),
        };

        let (utxos,): (UpdateBalanceResult,) = call(self.canister_id(), "update_balance", (args,))
            .await
            .map_err(|err| MinterError::CallError(err.1))?;

        Ok(utxos)
    }

    pub async fn retrieve_btc(
        &self,
        address: String,
        amount: u64,
    ) -> Result<RetrieveBtcResult, MinterError> {
        let args = RetrieveBtcArgs { address, amount };

        let (block_index,): (RetrieveBtcResult,) =
            call(self.canister_id(), "retrieve_btc", (args,))
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(block_index)
    }

    pub async fn retrieve_btc_status(
        &self,
        block_index: BlockIndex,
    ) -> Result<RetrieveBtcStatus, MinterError> {
        let args = RetrieveBtcStatusRequest { block_index };

        let (status,): (RetrieveBtcStatus,) =
            call(self.canister_id(), "retrieve_btc_status", (args,))
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(status)
    }
}

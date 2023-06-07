use super::types::{
    GetBtcAddressArgs, MinterError, RetrieveBtcArgs, RetrieveBtcResult, RetrieveBtcStatus,
    RetrieveBtcStatusRequest, UpdateBalanceArgs, UpdateBalanceResult,
};
use b3_helper_lib::{
    account::ICRCAccount,
    constants::CKBTC_MINTER_CANISTER,
    subaccount::Subaccount,
    types::{BlockIndex, CanisterId},
};
use ic_cdk::api::call::call;

pub struct Minter;

impl Minter {
    pub async fn get_btc_address(
        owner: Option<CanisterId>,
        subaccount: Option<Subaccount>,
    ) -> Result<String, MinterError> {
        let args = GetBtcAddressArgs { owner, subaccount };

        let (btc_address,): (String,) = call(CKBTC_MINTER_CANISTER, "get_btc_address", (args,))
            .await
            .map_err(|err| MinterError::CallError(err.1))?;

        Ok(btc_address)
    }

    pub async fn get_withdrawal_account() -> Result<ICRCAccount, MinterError> {
        let (withdrawal_account,): (ICRCAccount,) =
            call(CKBTC_MINTER_CANISTER, "get_withdrawal_account", ())
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(withdrawal_account)
    }

    pub async fn update_balance(
        owner: Option<CanisterId>,
        subaccount: Option<Subaccount>,
    ) -> Result<UpdateBalanceResult, MinterError> {
        let args = UpdateBalanceArgs { owner, subaccount };

        let (utxos,): (UpdateBalanceResult,) =
            call(CKBTC_MINTER_CANISTER, "update_balance", (args,))
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(utxos)
    }

    pub async fn retrieve_btc(
        address: String,
        amount: u64,
    ) -> Result<RetrieveBtcResult, MinterError> {
        let args = RetrieveBtcArgs { address, amount };

        let (block_index,): (RetrieveBtcResult,) =
            call(CKBTC_MINTER_CANISTER, "retrieve_btc", (args,))
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(block_index)
    }

    pub async fn retrieve_btc_status(
        block_index: BlockIndex,
    ) -> Result<RetrieveBtcStatus, MinterError> {
        let args = RetrieveBtcStatusRequest { block_index };

        let (status,): (RetrieveBtcStatus,) =
            call(CKBTC_MINTER_CANISTER, "retrieve_btc_status", (args,))
                .await
                .map_err(|err| MinterError::CallError(err.1))?;

        Ok(status)
    }
}

use b3_helper_lib::{
    constants::LEDGER_CANISTER_ID,
    types::{Memo, Timestamp},
};
use ic_cdk::api::call::call;

use crate::{error::WalletError, ledger::types::Ledger};

use super::{
    account::IcrcAccount,
    types::{ICRC1TransferArgs, TxIndex},
};

impl Ledger {
    pub async fn icrc_transfer(
        &self,
        to: IcrcAccount,
        amount: u64,
        fee: Option<u64>,
        memo: Option<Memo>,
        created_at_time: Option<Timestamp>,
    ) -> Result<TxIndex, WalletError> {
        let args = ICRC1TransferArgs {
            memo,
            fee,
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time,
        };

        let (res,): (TxIndex,) = call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res)
    }
}

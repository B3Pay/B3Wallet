use super::{
    icrc::IcrcChain,
    types::{ICRC1TransferArgs, TxIndex},
};
use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, PendingEnum, SendResult},
};
use async_trait::async_trait;
use b3_helper_lib::{account::ICRCAccount, amount::TokenAmount};
use std::str::FromStr;

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

#[async_trait]
impl ChainTrait for IcrcChain {
    fn address(&self) -> String {
        let owner = ic_cdk_id();

        self.subaccount.icrc_account(owner).to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let account = self.subaccount.icrc_account(ic_cdk_id());

        let (res,): (Balance,) = ic_cdk::call(self.canister_id, "icrc1_balance_of", (account,))
            .await
            .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(res)
    }

    async fn send(&self, to: String, amount: TokenAmount) -> Result<SendResult, LedgerError> {
        let to = ICRCAccount::from_str(&to).map_err(|e| LedgerError::CallError(e.to_string()))?;

        let transfer_args = ICRC1TransferArgs {
            to,
            amount: amount.to_nat(),
            from_subaccount: Some(self.subaccount.clone()),
            fee: self.fee.clone(),
            memo: self.memo.clone(),
            created_at_time: self.created_at_time,
        };

        let (res,): (TxIndex,) = ic_cdk::call(self.canister_id, "icrc1_transfer", (transfer_args,))
            .await
            .map_err(|e| LedgerError::CallError(e.1))?;

        Ok(SendResult::ICRC(res))
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: TokenAmount,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        // TODO: implement the update of the fee and memo fields if user wants to change them

        self.send(to, amount).await
    }

    async fn check_pending(&self, _pending_index: usize) -> Result<(), LedgerError> {
        Ok(())
    }

    fn pendings(&self) -> Vec<PendingEnum> {
        self.pendings
            .iter()
            .map(|pending| PendingEnum::ICRC(pending.clone()))
            .collect()
    }

    fn add_pending(&mut self, pending: PendingEnum) {
        if let PendingEnum::ICRC(p) = pending {
            self.pendings.push(p);
        }
    }

    fn remove_pending(&mut self, pending_index: usize) {
        self.pendings.remove(pending_index);
    }

    fn clear_pending(&mut self) {
        self.pendings.clear();
    }
}

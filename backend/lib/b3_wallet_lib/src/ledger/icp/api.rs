use async_trait::async_trait;
use b3_helper_lib::{identifier::AccountIdentifier, tokens::Tokens, types::NotifyTopUpResult};
use candid::Principal;
use std::str::FromStr;

#[cfg(test)]
use crate::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

use super::{error::IcpError, icp::IcpChain};
use crate::ledger::{
    chain::ChainTrait,
    error::LedgerError,
    types::{Balance, IcpPending, PendingEnum, SendResult},
};

#[async_trait]
impl ChainTrait for IcpChain {
    fn address(&self) -> String {
        self.subaccount.account_identifier(ic_cdk_id()).to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let canister_id = ic_cdk_id();

        let account = self.subaccount.account_identifier(canister_id);

        let res = self
            .account_balance(account)
            .await
            .map_err(|e| LedgerError::CallError(e.to_string()))?;

        Ok(res.e8s().into())
    }

    async fn send(&self, to: String, amount: u64) -> Result<SendResult, LedgerError> {
        let to =
            AccountIdentifier::from_str(&to).map_err(|e| LedgerError::CallError(e.to_string()))?;

        let amount = Tokens::from_e8s(amount);

        let res = self
            .transfer(to, amount, None, None)
            .await
            .map_err(|e| LedgerError::CallError(e.to_string()))?;

        Ok(SendResult::ICP(res))
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: u64,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        // TODO: This is a hack to get around the fact that we can't have mutable self and
        self.send(to, amount).await
    }

    async fn check_pending(&self, pending_index: usize) -> Result<(), LedgerError> {
        let IcpPending {
            block_index,
            canister_id,
        } = self
            .pendings
            .get(pending_index)
            .ok_or(LedgerError::PendingIndexError(pending_index))?;

        let canister_id = Principal::from_text(canister_id.clone())
            .map_err(|e| LedgerError::CallError(e.to_string()))?;

        let res = self
            .notify_top_up(canister_id, block_index.clone())
            .await
            .map_err(|e| LedgerError::CallError(e.to_string()))?;

        match res {
            NotifyTopUpResult::Ok(_) => Ok(()),
            NotifyTopUpResult::Err(err) => Err(LedgerError::IcpError(IcpError::NotifyError(err))),
        }
    }

    fn pendings(&self) -> Vec<PendingEnum> {
        self.pendings
            .iter()
            .map(|pending| PendingEnum::ICP(pending.clone()))
            .collect()
    }

    fn add_pending(&mut self, pending: PendingEnum) {
        if let PendingEnum::ICP(p) = pending {
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

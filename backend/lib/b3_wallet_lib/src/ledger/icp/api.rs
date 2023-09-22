use async_trait::async_trait;
use b3_utils::ledger::{
    currency::TokenAmount,
    types::{ICPTransferResult, NotifyTopUpResult},
    AccountIdentifier,
};
use candid::Principal;
use std::str::FromStr;

#[cfg(test)]
use b3_utils::mocks::id_mock as ic_cdk_id;
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
        let canister_id = ic_cdk_id();

        let account = AccountIdentifier::new(canister_id, self.subaccount.clone());

        account.to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let canister_id = ic_cdk_id();

        let account = AccountIdentifier::new(canister_id, self.subaccount.clone());

        let res = self
            .account_balance(account)
            .await
            .map_err(|e| LedgerError::CallError(e.to_string()))?;

        Ok(res.e8s().into())
    }

    async fn send(&self, to: String, amount: TokenAmount) -> Result<SendResult, LedgerError> {
        let to =
            AccountIdentifier::from_str(&to).map_err(|e| LedgerError::CallError(e.to_string()))?;

        let amount = amount
            .to_tokens()
            .map_err(|e| LedgerError::InvalidAmountError(e.to_string()))?;

        let result = self
            .transfer(to, amount, None, None)
            .await
            .map_err(|e| LedgerError::CallError(e.to_string()))?;

        match result {
            ICPTransferResult::Ok(block_index) => Ok(SendResult::ICP(block_index)),
            ICPTransferResult::Err(err) => {
                return Err(LedgerError::IcpError(IcpError::ICPTransferError(err)))
            }
        }
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

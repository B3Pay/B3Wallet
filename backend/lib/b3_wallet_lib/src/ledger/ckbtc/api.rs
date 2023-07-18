use super::{ckbtc::CkbtcChain, error::CkbtcError, types::RetrieveBtcStatus};
use crate::{
    ledger::types::{Balance, SendResult},
    ledger::{
        chain::ChainTrait,
        error::LedgerError,
        icrc::{error::IcrcError, types::ICRC1TransferArgs},
        types::{CkbtcPending, PendingEnum},
    },
};
use async_trait::async_trait;
use b3_helper_lib::{currency::TokenAmount, ICRCAccount};
use std::str::FromStr;

#[async_trait]
impl ChainTrait for CkbtcChain {
    fn address(&self) -> String {
        self.account.to_string()
    }

    async fn balance(&self) -> Result<Balance, LedgerError> {
        let account = self.account.clone();

        let result = self.ledger.balance_of(account).await;

        match result {
            Ok(balance) => Ok(balance),
            Err(err) => Err(LedgerError::IcrcError(err)),
        }
    }

    async fn send(&self, to: String, amount: TokenAmount) -> Result<SendResult, LedgerError> {
        let to = ICRCAccount::from_str(&to).map_err(LedgerError::ICRCAccountError)?;

        let transfer_args = ICRC1TransferArgs {
            to,
            amount: amount.to_nat(),
            from_subaccount: self.account.subaccount(),
            fee: self.fee.clone(),
            memo: self.memo.clone(),
            created_at_time: self.created_at_time,
        };

        let result = self
            .ledger
            .transfer(transfer_args)
            .await
            .map_err(LedgerError::IcrcError)?;

        match result {
            Ok(tx_index) => Ok(SendResult::CKBTC(tx_index)),
            Err(err) => Err(LedgerError::IcrcError(IcrcError::ICRC1TransferError(err))),
        }
    }

    async fn send_mut(
        &mut self,
        to: String,
        amount: TokenAmount,
        _fee: Option<u64>,
        _memo: Option<String>,
    ) -> Result<SendResult, LedgerError> {
        // TODO: update the struct if the user want that
        self.send(to, amount).await
    }

    async fn check_pending(&self, pending_index: usize) -> Result<(), LedgerError> {
        let CkbtcPending {
            block_index,
            txid: _,
        } = self
            .pendings
            .get(pending_index)
            .ok_or(LedgerError::PendingIndexError(pending_index))?;

        let result = self
            .minter
            .retrieve_btc_status(block_index.clone())
            .await
            .map_err(|err| LedgerError::CkbtcError(CkbtcError::MinterError(err)))?;

        match result {
            RetrieveBtcStatus::Confirmed { txid: _ } => Ok(()),
            _ => Err(LedgerError::CkbtcError(CkbtcError::RetrieveBtcStatus(
                result,
            ))),
        }
    }

    fn pendings(&self) -> Vec<PendingEnum> {
        self.pendings
            .iter()
            .map(|pending| PendingEnum::CKBTC(pending.clone()))
            .collect()
    }

    fn add_pending(&mut self, pending: PendingEnum) {
        if let PendingEnum::CKBTC(p) = pending {
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

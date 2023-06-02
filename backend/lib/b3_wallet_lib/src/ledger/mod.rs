pub mod btc;
pub mod chains;
pub mod config;
pub mod evm;
pub mod icrc;
pub mod keys;
pub mod subaccount;
pub mod types;

use crate::error::WalletError;
use b3_helper_lib::{
    constants::{
        CANISTER_TOP_UP_MEMO, CANISTER_TRANSFER_MEMO, CYCLES_MINTING_CANISTER_ID,
        IC_TRANSACTION_FEE_ICP, LEDGER_CANISTER_ID, MANAGMENT_CANISTER_ID,
    },
    error::TrapError,
    types::{
        AccountBalanceArgs, AccountIdentifier, CanisterId, Memo, NotifyTopUpResult,
        NotifyTopupArgs, Subaccount, Tokens, TransferArgs, TransferResult,
    },
};
use bitcoin::secp256k1::ecdsa::Signature;
use ic_cdk::{
    api::call::{call, call_with_payment},
    export::{candid::CandidType, serde::Deserialize},
};
use keys::Keys;
use subaccount::SubaccountTrait;
use types::{ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAResponse};

#[derive(CandidType, Clone, Deserialize)]
pub struct Ledger {
    pub keys: Keys,
    pub subaccount: Subaccount,
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            keys: Keys::default(),
            subaccount: Subaccount::default(),
        }
    }
}

impl From<Subaccount> for Ledger {
    fn from(subaccount: Subaccount) -> Self {
        let public_keys = Keys::from(subaccount.clone());

        Ledger {
            subaccount,
            keys: public_keys,
        }
    }
}

impl Ledger {
    pub fn set_ecdsa_public_key(&mut self, public_key: Vec<u8>) -> Result<(), WalletError> {
        let env = self.subaccount.environment();

        self.keys.set_ecdsa(public_key, env)
    }

    pub async fn ecdsa_public_key(&self) -> Result<Vec<u8>, WalletError> {
        let key_id = self.subaccount.key_id();

        let derivation_path = self.subaccount.derivation_path();

        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path,
            key_id,
        };

        let (res,): (ECDSAPublicKeyResponse,) =
            call(MANAGMENT_CANISTER_ID, "ecdsa_public_key", (request,))
                .await
                .map_err(|e| WalletError::PublicKeyError(e.1))?;

        Ok(res.public_key)
    }

    pub async fn sign_btc_transaction(
        &self,
        message_hash: Vec<u8>,
    ) -> Result<Signature, WalletError> {
        let sig = self.sign_with_ecdsa(message_hash).await?;

        let signature =
            Signature::from_compact(&sig).map_err(|err| WalletError::SignError(err.to_string()))?;

        Ok(signature)
    }

    pub async fn sign_with_ecdsa(&self, message_hash: Vec<u8>) -> Result<Vec<u8>, WalletError> {
        let (key_id, cycles, derivation_path) = self.subaccount.key_id_with_cycles_and_path();

        let request = SignWithECDSAArgs {
            derivation_path,
            message_hash,
            key_id,
        };

        let (res,): (SignWithECDSAResponse,) =
            call_with_payment(MANAGMENT_CANISTER_ID, "sign_with_ecdsa", (request,), cycles)
                .await
                .map_err(|e| WalletError::SignError(e.1))?;

        Ok(res.signature)
    }

    pub async fn account_balance(&self, owner: Option<CanisterId>) -> Result<Tokens, WalletError> {
        let owner = owner.unwrap_or(ic_cdk::id());

        let account = self.subaccount.account_identifier(owner);

        let args = AccountBalanceArgs { account };

        let (res,): (Tokens,) = call(LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res)
    }

    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: Tokens,
        fee: Option<Tokens>,
        memo: Option<Memo>,
    ) -> Result<TransferResult, WalletError> {
        let args = TransferArgs {
            memo: memo.unwrap_or(CANISTER_TRANSFER_MEMO),
            fee: fee.unwrap_or(IC_TRANSACTION_FEE_ICP),
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (TransferResult,) = call(LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| WalletError::LedgerError(e.1))?;

        Ok(res)
    }

    pub async fn topup_and_notify_top_up(
        &self,
        canister_id: CanisterId,
        amount: Tokens,
        fee: Option<Tokens>,
    ) -> Result<NotifyTopUpResult, WalletError> {
        let canister_subaccount = Subaccount::from(canister_id);

        let to = AccountIdentifier::new(CYCLES_MINTING_CANISTER_ID, canister_subaccount);

        let block_index = self
            .transfer(to, amount, fee, Some(CANISTER_TOP_UP_MEMO))
            .await?
            .map_err(|e| WalletError::LedgerError(e.to_string()))?;

        let args = NotifyTopupArgs {
            block_index,
            canister_id,
        };

        let (res,): (NotifyTopUpResult,) =
            call(CYCLES_MINTING_CANISTER_ID, "notify_top_up", (args,))
                .await
                .map_err(|e| WalletError::CyclesMintingError(e.1))?;

        Ok(res)
    }
}

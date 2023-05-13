use ic_cdk::{
    api::call::{call, call_with_payment},
    export::{candid::CandidType, serde::Deserialize},
};

use crate::{error::SignerError, ledger::public_keys::PublicKeys};
use b3_helper::{
    constants::{
        CANISTER_TOP_UP_MEMO, CANISTER_TRANSFER_MEMO, IC_TRANSACTION_FEE_ICP,
        MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID,
        MAINNET_MANAGMENT_CANISTER_ID,
    },
    error::TrapError,
    types::{
        AccountBalanceArgs, AccountIdentifier, CanisterId, Memo, NotifyTopUpResult,
        NotifyTopupArgs, Subaccount, Tokens, TransferArgs, TransferResult,
    },
};

use super::{
    subaccount::SubaccountTrait,
    types::{ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAResponse},
};

#[derive(CandidType, Clone, Deserialize)]
pub struct Ledger {
    pub public_keys: PublicKeys,
    pub subaccount: Subaccount,
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            public_keys: PublicKeys::default(),
            subaccount: Subaccount::default(),
        }
    }
}

impl From<Subaccount> for Ledger {
    fn from(subaccount: Subaccount) -> Self {
        let public_keys = subaccount.clone().into();

        Ledger {
            subaccount,
            public_keys,
        }
    }
}

impl Ledger {
    pub async fn ecdsa_public_key(&self) -> Result<Vec<u8>, SignerError> {
        let key_id = self.subaccount.key_id();

        let derivation_path = self.subaccount.derivation_path();

        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path,
            key_id,
        };

        let (res,): (ECDSAPublicKeyResponse,) = call(
            MAINNET_MANAGMENT_CANISTER_ID,
            "ecdsa_public_key",
            (request,),
        )
        .await
        .map_err(|e| SignerError::PublicKeyError(e.1))?;

        Ok(res.public_key)
    }

    pub async fn sign_with_ecdsa(&self, message_hash: Vec<u8>) -> Result<Vec<u8>, SignerError> {
        let (key_id, cycles, derivation_path) = self.subaccount.key_id_with_cycles_and_path();

        let request = SignWithECDSAArgs {
            derivation_path,
            message_hash,
            key_id,
        };

        let (res,): (SignWithECDSAResponse,) = call_with_payment(
            MAINNET_MANAGMENT_CANISTER_ID,
            "sign_with_ecdsa",
            (request,),
            cycles,
        )
        .await
        .map_err(|e| SignerError::SignError(e.1))?;

        Ok(res.signature)
    }

    pub async fn account_balance(&self) -> Result<Tokens, SignerError> {
        let account = self.subaccount.account_identifier();

        let args = AccountBalanceArgs { account };

        let (res,): (Tokens,) = call(MAINNET_LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| SignerError::LedgerError(e.1))?;

        Ok(res)
    }

    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: Tokens,
        fee: Option<Tokens>,
        memo: Option<Memo>,
    ) -> Result<TransferResult, SignerError> {
        let args = TransferArgs {
            memo: memo.unwrap_or(CANISTER_TRANSFER_MEMO),
            fee: fee.unwrap_or(IC_TRANSACTION_FEE_ICP),
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let (res,): (TransferResult,) = call(MAINNET_LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| SignerError::LedgerError(e.1))?;

        Ok(res)
    }

    pub async fn topup_and_notify_top_up(
        &self,
        canister_id: CanisterId,
        amount: Tokens,
        fee: Option<Tokens>,
    ) -> Result<NotifyTopUpResult, SignerError> {
        let canister_subaccount: Subaccount = canister_id.into();

        let to = AccountIdentifier::new(&MAINNET_CYCLES_MINTING_CANISTER_ID, &canister_subaccount);

        let block_index = self
            .transfer(to, amount, fee, Some(CANISTER_TOP_UP_MEMO))
            .await?
            .map_err(|e| SignerError::LedgerError(e.to_string()))?;

        let args = NotifyTopupArgs {
            block_index,
            canister_id,
        };

        let (res,): (NotifyTopUpResult,) =
            call(MAINNET_CYCLES_MINTING_CANISTER_ID, "notify_top_up", (args,))
                .await
                .map_err(|e| SignerError::CyclesMintingError(e.1))?;

        Ok(res)
    }
}

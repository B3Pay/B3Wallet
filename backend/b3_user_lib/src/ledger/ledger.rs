use ic_cdk::{
    api::call::CallResult,
    export::{candid::CandidType, serde::Deserialize, Principal},
};

use crate::{
    allowance::CanisterId,
    error::SignerError,
    ledger::{identifier::AccountIdentifier, keys::Keys, subaccount::Subaccount},
    types::{
        ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, Memo, NotifyTopUpResult, Timestamp,
        TransferResult, MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID,
        MAINNET_MANAGMENT_CANISTER_ID,
    },
};

use ic_cdk::api::call::{call, call_with_payment};

use crate::types::{SignWithECDSAArgs, SignWithECDSAResponse};

use super::constants::{CANISTER_TOP_UP_MEMO, IC_TRANSACTION_FEE_ICP};

pub type BlockIndex = u64;

#[derive(CandidType, Deserialize)]
pub struct NotifyTopupArgs {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

#[derive(CandidType)]
pub struct AccountBalanceArgs {
    pub account: AccountIdentifier,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Tokens {
    pub e8s: u64,
}

impl Tokens {
    /// The maximum number of Tokens we can hold on a single account.
    pub const MAX: Self = Tokens { e8s: u64::MAX };
    /// Zero Tokens.
    pub const ZERO: Self = Tokens { e8s: 0 };
    /// How many times can Tokenss be divided
    pub const SUBDIVIDABLE_BY: u64 = 100_000_000;

    /// Constructs an amount of Tokens from the number of 10^-8 Tokens.
    pub const fn from_e8s(e8s: u64) -> Self {
        Self { e8s }
    }

    /// Returns the number of 10^-8 Tokens in this amount.
    pub const fn e8s(&self) -> u64 {
        self.e8s
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub memo: Memo,
    pub fee: Tokens,
    pub amount: Tokens,
    pub to: AccountIdentifier,
    pub from_subaccount: Option<Subaccount>,
    pub created_at_time: Option<Timestamp>,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
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

impl Ledger {
    pub async fn new(subaccount: Subaccount) -> CallResult<Self> {
        let identifier = subaccount.get_account_identifier();

        let icp = identifier.to_str();

        let mut ledger = Ledger {
            keys: Keys::default(),
            subaccount,
        };

        let bytes = ledger.public_key().await?;

        ledger.keys = Keys::new(bytes, icp);

        Ok(ledger)
    }

    pub async fn public_key(&self) -> CallResult<Vec<u8>> {
        let key_id = self.subaccount.get_key_id();

        let derivation_path = self.subaccount.get_derivation_path();

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

    pub async fn sign_message(&self, message_hash: Vec<u8>) -> CallResult<Vec<u8>> {
        let (key_id, cycles, derivation_path) = self.subaccount.get_key_id_with_cycles_and_path();

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

    pub async fn account_balance(&self) -> CallResult<Tokens> {
        let account = self.subaccount.get_account_identifier();

        let args = AccountBalanceArgs { account };

        let (res,): (Tokens,) = call(MAINNET_LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| SignerError::LedgerError(e.1))?;

        Ok(res)
    }

    pub async fn transfer_icp(
        &self,
        amount: Tokens,
        to: AccountIdentifier,
        fee: Option<Tokens>,
        memo: Option<Memo>,
    ) -> CallResult<TransferResult> {
        let args = TransferArgs {
            memo: memo.unwrap_or_else(|| Memo(1234567890)),
            fee: fee.unwrap_or_else(|| IC_TRANSACTION_FEE_ICP),
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

    pub async fn topup_and_notify(
        &self,
        amount: Tokens,
        canister_id: CanisterId,
        fee: Option<Tokens>,
    ) -> CallResult<NotifyTopUpResult> {
        let canister_subaccount = Subaccount::from(&canister_id);

        let to = AccountIdentifier::new(&MAINNET_CYCLES_MINTING_CANISTER_ID, &canister_subaccount);

        let block_index = self
            .transfer_icp(amount, to, fee, Some(CANISTER_TOP_UP_MEMO))
            .await?
            .unwrap();

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

use ic_cdk::{
    api::call::CallResult,
    export::{candid::CandidType, serde::Deserialize, Principal},
};

use crate::{
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
        let owner = ic_cdk::caller();

        let identifier = AccountIdentifier::new(&owner, &subaccount);

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

    pub async fn sign(&self, message_hash: Vec<u8>) -> CallResult<Vec<u8>> {
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

    pub async fn transfer_icp(
        &self,
        amount: Tokens,
        to: AccountIdentifier,
        fee: Option<Tokens>,
        memo: Option<Memo>,
    ) -> CallResult<TransferResult> {
        let args = TransferArgs {
            memo: memo.unwrap_or_else(|| Memo(1234567890)),
            fee: fee.unwrap_or_else(|| Tokens { e8s: 10_000 }),
            amount,
            to,
            from_subaccount: Some(self.subaccount.clone()),
            created_at_time: None,
        };

        let response: (TransferResult,) = call(MAINNET_LEDGER_CANISTER_ID, "transfer", (args,))
            .await
            .map_err(|e| SignerError::LedgerError(e.1))?;

        Ok(response.0)
    }

    pub async fn account_balance(&self) -> CallResult<Tokens> {
        let account = self.subaccount.account_identifier();

        let args = AccountBalanceArgs { account };

        let response: (Tokens,) = call(MAINNET_LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| SignerError::LedgerError(e.1))?;

        Ok(response.0)
    }

    pub async fn notify_topup(&self, args: NotifyTopupArgs) -> CallResult<NotifyTopUpResult> {
        let response: (NotifyTopUpResult,) =
            call(MAINNET_CYCLES_MINTING_CANISTER_ID, "notify_topup", (args,))
                .await
                .map_err(|e| SignerError::CyclesMintingError(e.1))?;

        Ok(response.0)
    }
}

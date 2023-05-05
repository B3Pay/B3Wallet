use ic_cdk::{
    api::call::CallResult,
    export::{candid::CandidType, serde::Deserialize, Principal},
};
use std::mem::size_of;

use crate::{
    error::SignerError,
    ledger::{ecdsa::Ecdsa, identifier::AccountIdentifier, keys::Keys},
    types::{
        Memo, NotifyTopUpResult, Timestamp, TransferResult, MAINNET_CYCLES_MINTING_CANISTER_ID,
        MAINNET_LEDGER_CANISTER_ID, MAINNET_MANAGMENT_CANISTER_ID,
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
pub struct Subaccount(pub [u8; 32]);

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0; 32])
    }
}

impl From<&Vec<u8>> for Subaccount {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut subaccount = [0; size_of::<Subaccount>()];
        subaccount[0] = bytes.len().try_into().unwrap();
        subaccount[1..1 + bytes.len()].copy_from_slice(&bytes[..]);
        Subaccount(subaccount)
    }
}

impl From<&Principal> for Subaccount {
    fn from(principal_id: &Principal) -> Self {
        let mut subaccount = [0; size_of::<Subaccount>()];
        let principal_id = principal_id.as_slice();

        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

        Subaccount(subaccount)
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
    pub ecdsa: Ecdsa,
    pub subaccount: Subaccount,
    pub identifier: AccountIdentifier,
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            keys: Keys::default(),
            ecdsa: Ecdsa::default(),
            subaccount: Subaccount::default(),
            identifier: AccountIdentifier::default(),
        }
    }
}

impl Ledger {
    pub async fn new(ecdsa: Ecdsa) -> CallResult<Self> {
        let owner = ic_cdk::caller();

        let path = ecdsa.path();

        let subaccount = Subaccount::from(&path);

        let identifier = AccountIdentifier::new(&owner, &subaccount);

        let icp = identifier.to_str();

        let bytes = ecdsa.public_key().await;

        let keys = Keys::new(bytes, icp);

        Ok(Self {
            keys,
            ecdsa,
            subaccount,
            identifier,
        })
    }

    pub async fn sign(&self, message_hash: Vec<u8>) -> CallResult<Vec<u8>> {
        let (key_id, cycles, derivation_path) = self.ecdsa.key_id_with_cycles_and_path();

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

    pub async fn transfer(
        &self,
        to: AccountIdentifier,
        amount: Tokens,
    ) -> CallResult<TransferResult> {
        let args = TransferArgs {
            memo: Memo(1234567890),
            fee: Tokens { e8s: 10_000 },
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

    pub async fn account_balance(account: AccountIdentifier) -> CallResult<Tokens> {
        let args = AccountBalanceArgs { account };

        let response: (Tokens,) = call(MAINNET_LEDGER_CANISTER_ID, "account_balance", (args,))
            .await
            .map_err(|e| SignerError::LedgerError(e.1))?;

        Ok(response.0)
    }

    pub async fn notify_topup(args: NotifyTopupArgs) -> CallResult<NotifyTopUpResult> {
        let response: (NotifyTopUpResult,) =
            call(MAINNET_CYCLES_MINTING_CANISTER_ID, "notify_topup", (args,))
                .await
                .map_err(|e| SignerError::CyclesMintingError(e.1))?;

        Ok(response.0)
    }
}

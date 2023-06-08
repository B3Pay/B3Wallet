use super::ledger::Ledger;
use super::subaccount::SubaccountTrait;
use super::types::{
    ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAResponse,
};
use crate::error::WalletError;
use b3_helper_lib::constants::MANAGMENT_CANISTER_ID;
use bitcoin::secp256k1::ecdsa::Signature;
use ic_cdk::api::call::{call, call_with_payment};

impl Ledger {
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
}

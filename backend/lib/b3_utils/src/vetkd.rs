mod error;
pub use error::*;

mod types;
use ic_cdk::api::call::{call, call_with_payment};
pub use types::*;

mod config;
pub use config::*;

use crate::{constants::MANAGMENT_CANISTER_ID, Environment, Subaccount};

pub struct VetKD(Subaccount);

impl VetKD {
    pub fn new(subaccount: Subaccount) -> Self {
        Self(subaccount)
    }

    pub fn environment(&self) -> Environment {
        self.0.environment()
    }

    pub fn derivation_path(&self) -> Vec<Vec<u8>> {
        self.0.derivation_path()
    }

    pub fn derivation_id(&self) -> Vec<u8> {
        self.0.derivation_id()
    }

    pub fn config(&self) -> VetKDConfig {
        self.0.environment().into()
    }

    pub fn key_id(&self) -> VetKDKeyId {
        self.config().key_id()
    }

    pub fn key_id_with_cycles_and_path(&self) -> (VetKDKeyId, u64, Vec<Vec<u8>>) {
        let config = self.config();

        (
            config.key_id(),
            config.sign_cycles(),
            self.derivation_path(),
        )
    }

    pub async fn verification_key(&self) -> Result<Vec<u8>, VetKDError> {
        let key_id = self.key_id();
        let derivation_path = self.derivation_path();

        let request = VetKDPublicKeyRequest {
            canister_id: None,
            derivation_path,
            key_id,
        };

        let (res,): (VetKDPublicKeyReply,) =
            call(MANAGMENT_CANISTER_ID, "vetkd_public_key", (request,))
                .await
                .map_err(|e| VetKDError::CallError(e.1))?;

        Ok(res.public_key)
    }

    pub async fn encrypted_key(
        &self,
        encryption_public_key: Vec<u8>,
    ) -> Result<Vec<u8>, VetKDError> {
        let key_id = self.key_id();
        let public_key_derivation_path = self.derivation_path();
        let derivation_id = self.derivation_id();

        let request = VetKDEncryptedKeyRequest {
            derivation_id,
            public_key_derivation_path,
            key_id,
            encryption_public_key,
        };

        let (res,): (VetKDEncryptedKeyReply,) =
            call_with_payment(MANAGMENT_CANISTER_ID, "vetkd_encrypted_key", (request,), 0)
                .await
                .map_err(|e| VetKDError::CallError(e.1))?;

        Ok(res.encrypted_key)
    }
}

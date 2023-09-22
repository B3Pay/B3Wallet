mod error;
pub use error::*;

use ic_cdk::api::call::{call, call_with_payment};

pub mod types;

mod pairing;
pub use pairing::*;

mod config;
pub use config::*;

use crate::{environment::Environment, types::CanisterId, Subaccount};

use self::types::*;

/// TODO: Replace with the real MANAGMENT_CANISTER_ID from crate::constants
/// this is the canister id of an unsafe VETKD
/// canister, it is used for testing purposes only
/// and should be replaced with the MANAGE_CANISTER_ID
fn vetkd_system_api_canister_id() -> CanisterId {
    CanisterId::from_text("wfdtj-lyaaa-aaaap-abakq-cai")
        .expect("Error::Failed to create canister ID")
}

pub struct VetKD(pub Subaccount);

impl VetKD {
    pub fn new(subaccount: Subaccount) -> Self {
        Self(subaccount)
    }

    pub fn environment(&self) -> Environment {
        self.0.environment()
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

    pub fn key_id_with_cycles(&self) -> (VetKDKeyId, u64) {
        let config = self.config();

        (config.key_id(), config.sign_cycles())
    }

    pub async fn request_encrypted_key(
        &self,
        public_key_derivation_path: Vec<Vec<u8>>,
        encryption_public_key: Vec<u8>,
    ) -> Result<Vec<u8>, VetKDError> {
        let key_id = self.key_id();
        let derivation_id = self.derivation_id();

        let request = VetKDEncryptedKeyRequest {
            derivation_id,
            public_key_derivation_path,
            key_id,
            encryption_public_key,
        };

        let (res,): (VetKDEncryptedKeyReply,) = call_with_payment(
            vetkd_system_api_canister_id(),
            "vetkd_encrypted_key",
            (request,),
            0,
        )
        .await
        .map_err(|e| VetKDError::CallError(e.1))?;

        Ok(res.encrypted_key)
    }
}

pub struct VetKDManagement(pub CanisterId);

impl VetKDManagement {
    pub fn new(canister_id: CanisterId) -> Self {
        Self(canister_id)
    }

    pub fn config(&self) -> VetKDConfig {
        Subaccount::from(self.0).environment().into()
    }

    pub fn key_id(&self) -> VetKDKeyId {
        self.config().key_id()
    }

    pub async fn request_public_key(
        &self,
        derivation_path: Vec<Vec<u8>>,
    ) -> Result<Vec<u8>, VetKDError> {
        let key_id = self.key_id();

        let request = VetKDPublicKeyRequest {
            canister_id: Some(self.0),
            derivation_path,
            key_id,
        };

        let (res,): (VetKDPublicKeyReply,) = call(
            vetkd_system_api_canister_id(),
            "vetkd_public_key",
            (request,),
        )
        .await
        .map_err(|e| VetKDError::CallError(e.1))?;

        Ok(res.public_key)
    }

    pub async fn request_encrypted_key(
        &self,
        derivation_id: Vec<u8>,
        public_key_derivation_path: Vec<Vec<u8>>,
        encryption_public_key: Vec<u8>,
    ) -> Result<Vec<u8>, VetKDError> {
        let key_id = self.key_id();

        let request = VetKDEncryptedKeyRequest {
            derivation_id,
            public_key_derivation_path,
            key_id,
            encryption_public_key,
        };

        let (res,): (VetKDEncryptedKeyReply,) = call_with_payment(
            vetkd_system_api_canister_id(),
            "vetkd_encrypted_key",
            (request,),
            0,
        )
        .await
        .map_err(|e| VetKDError::CallError(e.1))?;

        Ok(res.encrypted_key)
    }
}

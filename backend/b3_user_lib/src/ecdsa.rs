use ic_cdk::{
    export::{candid::CandidType, serde::Deserialize, Principal},
    trap,
};

use ic_cdk::api::call::call_with_payment as ic_call;

use crate::{
    config::{Config, Environment},
    types::{
        ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, EcdsaKeyId, SignWithECDSAArgs,
        SignWithECDSAResponse,
    },
};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Ecdsa {
    pub path: Vec<u8>,
    pub env: Environment,
}

impl Default for Ecdsa {
    fn default() -> Self {
        Ecdsa {
            path: Vec::new(),
            env: Environment::default(),
        }
    }
}

impl Ecdsa {
    pub fn new(path: Vec<u8>, env: Environment) -> Self {
        Ecdsa { path, env }
    }

    pub fn path_id(&self) -> String {
        let last = self.path.len() - 1;

        match self.env {
            Environment::Production => format!("key_{:x}", self.path[last]),
            _ => format!("test_key_{:x}", self.path[last]),
        }
    }

    pub fn config(&self) -> Config {
        Config::from(self.env.clone())
    }

    pub fn key_id(&self) -> EcdsaKeyId {
        let config = self.config();

        config.key_id()
    }

    pub async fn public_key(&self) -> Vec<u8> {
        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path: vec![self.path.clone()],
            key_id: self.key_id(),
        };

        let (res,): (ECDSAPublicKeyResponse,) = ic_call(
            Principal::management_canister(),
            "ecdsa_public_key",
            (request,),
            0 as u64,
        )
        .await
        .map_err(|e| trap(&format!("Failed to call ecdsa_public_key {}", e.1)))
        .unwrap();

        res.public_key
    }

    pub async fn sign_message(&self, message_hash: Vec<u8>) -> Vec<u8> {
        let request = SignWithECDSAArgs {
            derivation_path: vec![self.path.clone()],
            key_id: self.key_id(),
            message_hash,
        };

        let (res,): (SignWithECDSAResponse,) = ic_call(
            Principal::management_canister(),
            "sign_with_ecdsa",
            (request,),
            0 as u64,
        )
        .await
        .map_err(|e| trap(&format!("Failed to call sign_with_ecdsa {}", e.1)))
        .unwrap();

        res.signature
    }
}

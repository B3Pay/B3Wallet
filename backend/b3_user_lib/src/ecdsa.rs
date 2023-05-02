use ic_cdk::{
    export::{candid::CandidType, serde::Deserialize, Principal},
    trap,
};

use ic_cdk::api::call::{call, call_with_payment};

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

    pub fn config(&self) -> Config {
        Config::from(self.env.clone())
    }

    pub fn path_id(&self) -> String {
        let last = self.path.len() - 1;

        match self.env {
            Environment::Production => format!("account_{:x}", self.path[last]),
            _ => format!("test_account_{:x}", self.path[last]),
        }
    }

    pub fn key_id(&self) -> EcdsaKeyId {
        let config = self.config();

        config.key_id()
    }

    pub fn cycles(&self) -> u64 {
        let config = self.config();

        config.sign_cycles()
    }

    pub fn derivation_path(&self) -> Vec<Vec<u8>> {
        vec![self.path.clone()]
    }

    pub fn key_id_with_cycles(&self) -> (EcdsaKeyId, u64) {
        let config = self.config();

        (config.key_id(), config.sign_cycles())
    }

    pub async fn public_key(&self) -> Vec<u8> {
        let key_id = self.key_id();
        let derivation_path = self.derivation_path();

        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path,
            key_id,
        };

        let (res,): (ECDSAPublicKeyResponse,) = call(
            Principal::management_canister(),
            "ecdsa_public_key",
            (request,),
        )
        .await
        .map_err(|e| trap(&format!("Failed to call ecdsa_public_key {}", e.1)))
        .unwrap();

        res.public_key
    }

    pub async fn sign(&self, message_hash: Vec<u8>) -> Vec<u8> {
        let (key_id, cycles) = self.key_id_with_cycles();
        let derivation_path = self.derivation_path();

        let request = SignWithECDSAArgs {
            derivation_path,
            message_hash,
            key_id,
        };

        let (res,): (SignWithECDSAResponse,) = call_with_payment(
            Principal::management_canister(),
            "sign_with_ecdsa",
            (request,),
            cycles,
        )
        .await
        .map_err(|e| trap(&format!("Failed to call sign_with_ecdsa {}", e.1)))
        .unwrap();

        res.signature
    }
}

use ic_cdk::{
    api::call::call,
    export::{candid::CandidType, serde::Deserialize},
    trap,
};

use crate::{
    ledger::config::{Config, Environment},
    types::{
        ECDSAPublicKeyArgs, ECDSAPublicKeyResponse, EcdsaKeyId, MAINNET_MANAGMENT_CANISTER_ID,
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

    pub async fn public_key(&self) -> Vec<u8> {
        let key_id = self.key_id();
        let derivation_path = self.derivation_path();

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
        .map_err(|e| trap(&format!("Failed to call ecdsa_public_key {}", e.1)))
        .unwrap();

        res.public_key
    }

    pub fn config(&self) -> Config {
        Config::from(self.env.clone())
    }

    pub fn path(&self) -> Vec<u8> {
        self.path.clone()
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

    pub fn key_id_with_cycles_and_path(&self) -> (EcdsaKeyId, u64, Vec<Vec<u8>>) {
        let config = self.config();

        (
            config.key_id(),
            config.sign_cycles(),
            vec![self.path.clone()],
        )
    }
}

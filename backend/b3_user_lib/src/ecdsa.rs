use ic_cdk::{
    export::{
        candid::CandidType,
        serde::{Deserialize, Serialize},
        Principal,
    },
    trap,
};

#[cfg(test)]
use crate::mocks::ic_call;

#[cfg(not(test))]
use ic_cdk::api::call::call_with_payment as ic_call;

use crate::config::{Config, Environment};

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ECDSAPublicKeyResponse {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithECDSAResponse {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct ECDSAPublicKeyArgs {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug, Deserialize)]
pub struct SignWithECDSAArgs {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Ecdsa {
    path: Vec<u8>,
    config: Config,
}

impl Default for Ecdsa {
    fn default() -> Self {
        Ecdsa {
            path: Vec::new(),
            config: Config::default(),
        }
    }
}

impl Ecdsa {
    pub fn new(path: Vec<u8>, env: Environment) -> Self {
        let config = Config::from(env);

        Ecdsa { path, config }
    }

    pub async fn public_key(&self) -> Vec<u8> {
        let request = ECDSAPublicKeyArgs {
            canister_id: None,
            derivation_path: vec![self.path.clone()],
            key_id: self.config.key_id(),
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
            key_id: self.config.key_id(),
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

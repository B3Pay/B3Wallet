use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};

type CanisterId = Principal;

pub mod reply {
    use super::*;

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
}
pub mod request {
    use super::*;

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
}

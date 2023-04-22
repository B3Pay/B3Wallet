use crate::subaccount::Subaccount;
use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
use sha2::Digest;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AccountIdentifier(pub [u8; 32]);

impl AccountIdentifier {
    pub fn new(owner: &Principal, subaccount: &Subaccount) -> Self {
        let mut hasher = sha2::Sha224::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(owner.as_slice());
        hasher.update(&subaccount.0[..]);
        let hash: [u8; 28] = hasher.finalize().into();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&hash);
        let crc32_bytes = hasher.finalize().to_be_bytes();

        let mut result = [0u8; 32];
        result[0..4].copy_from_slice(&crc32_bytes[..]);
        result[4..32].copy_from_slice(hash.as_ref());

        Self(result)
    }
}

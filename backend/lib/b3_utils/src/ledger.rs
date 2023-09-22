mod base32;
pub mod currency;

mod icrc;
pub use icrc::*;

mod identifier;
pub use identifier::*;

pub mod types;

pub mod constants;

use ::easy_hasher::easy_hasher::Hash;
use easy_hasher::easy_hasher;

pub fn raw_keccak256(data: &[u8]) -> Hash {
    easy_hasher::raw_keccak256(data.to_vec())
}

pub fn raw_sha256(data: &[u8]) -> Vec<u8> {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    hash.to_vec()
}

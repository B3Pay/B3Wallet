use ::easy_hasher::easy_hasher::Hash;
use easy_hasher::easy_hasher;

pub fn raw_keccak256(data: &[u8]) -> Hash {
    easy_hasher::raw_keccak256(data.to_vec())
}

pub fn sha2_sha256(data: &[u8]) -> Vec<u8> {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    hash.to_vec()
}

pub fn vec_to_hex_string(data: &[u8]) -> String {
    hex::encode(data)
}

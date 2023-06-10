use b3_helper_lib::raw_keccak256;
use bitcoin::secp256k1::PublicKey;

pub trait PublicKeyTrait {
    fn to_address(&self) -> String;
    fn to_evm_key(&self) -> Vec<u8>; // Return owned Vec<u8> instead of reference
}

impl PublicKeyTrait for PublicKey {
    fn to_address(&self) -> String {
        let pub_key = self.serialize_uncompressed();

        let keccak256 = raw_keccak256(&pub_key[1..]);

        let keccak256_hex = keccak256.to_hex_string();

        let address = "0x".to_owned() + &keccak256_hex[24..];

        address
    }

    fn to_evm_key(&self) -> Vec<u8> {
        // Return owned Vec<u8> instead of reference
        let pub_key = self.serialize_uncompressed();

        let pub_key_hash = raw_keccak256(&pub_key[1..]).to_vec();

        let key = pub_key_hash[12..].to_vec();

        key
    }
}

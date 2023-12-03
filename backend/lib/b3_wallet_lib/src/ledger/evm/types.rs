use b3_utils::vec_to_hex_string_with_0x;
use bitcoin::secp256k1::PublicKey;
use tiny_keccak::{Hasher, Keccak};

pub trait PublicKeyTrait {
    fn to_address(&self) -> String;
    fn to_evm_key(&self) -> Vec<u8>; // Return owned Vec<u8> instead of reference
}

impl PublicKeyTrait for PublicKey {
    fn to_address(&self) -> String {
        let pub_key = self.serialize_uncompressed();

        let mut keccak = Keccak::v256();
        keccak.update(&pub_key[1..]);
        let mut output = [0u8; 32];
        keccak.finalize(&mut output);

        // Convert the last 20 bytes of hash to hex string
        let address = vec_to_hex_string_with_0x(&output[12..]);

        address
    }

    fn to_evm_key(&self) -> Vec<u8> {
        let pub_key = self.serialize_uncompressed();

        let mut keccak = Keccak::v256();
        keccak.update(&pub_key[1..]);
        let mut output = [0u8; 32];
        keccak.finalize(&mut output);

        // Return the last 20 bytes of the hash
        output[12..].to_vec()
    }
}

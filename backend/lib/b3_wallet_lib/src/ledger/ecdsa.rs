use b3_utils::vec_to_hex_string_with_0x;
use libsecp256k1::PublicKey;
use tiny_keccak::Hasher;
use tiny_keccak::Keccak;

use super::{
    btc::{address::network_and_public_key_to_p2wpkh, network::BitcoinNetwork},
    error::LedgerError,
};

pub trait ChainAddress {
    fn btc_address(&self, network: BitcoinNetwork) -> Result<String, LedgerError>;
    fn eth_address(&self) -> Result<String, LedgerError>;
}

impl ChainAddress for PublicKey {
    fn btc_address(&self, network: BitcoinNetwork) -> Result<String, LedgerError> {
        let address = network_and_public_key_to_p2wpkh(network, &self.serialize_compressed());

        Ok(address)
    }

    fn eth_address(&self) -> Result<String, LedgerError> {
        let mut hasher = Keccak::v256();

        hasher.update(&self.serialize()[1..]);

        let mut hash = [0u8; 32];

        hasher.finalize(&mut hash);

        let mut address = [0u8; 20];

        address.copy_from_slice(&hash[12..]);

        Ok(vec_to_hex_string_with_0x(&address))
    }
}

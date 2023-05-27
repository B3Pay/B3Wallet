use crate::error::WalletError;
use b3_helper_lib::raw_keccak256;
use b3_helper_lib::types::{AccountIdentifier, Subaccount};

use bitcoin::{secp256k1, Address, PublicKey};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::HashMap;

use super::btc::network::BtcNetwork;
use super::types::AddressMap;
use super::{chains::Chains, subaccount::SubaccountTrait, types::EcdsaPublicKey};

#[derive(CandidType, Deserialize, Clone)]
pub struct Keys {
    pub ecdsa: Option<EcdsaPublicKey>,
    pub addresses: AddressMap,
    pub identifier: AccountIdentifier,
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            ecdsa: None,
            addresses: HashMap::new(),
            identifier: AccountIdentifier::default(),
        }
    }
}

impl From<Subaccount> for Keys {
    fn from(subaccount: Subaccount) -> Self {
        let canister_id = ic_cdk::id();

        let identifier = subaccount.account_identifier(canister_id);

        let mut addresses = AddressMap::new();

        addresses.insert(Chains::ICP, identifier.to_string());

        Keys {
            ecdsa: None,
            identifier,
            addresses,
        }
    }
}

impl Keys {
    pub fn is_ecdsa_set(&self) -> bool {
        self.ecdsa
            .as_ref()
            .map(|ecdsa| ecdsa.len() == 33)
            .unwrap_or(false)
    }

    pub fn identifier(&self) -> &AccountIdentifier {
        &self.identifier
    }

    pub fn addresses(&self) -> &AddressMap {
        &self.addresses
    }

    pub fn ecdsa(&self) -> Result<&Vec<u8>, WalletError> {
        match &self.ecdsa {
            Some(ecdsa) => Ok(ecdsa),
            None => Err(WalletError::MissingEcdsaPublicKey),
        }
    }

    pub fn get_public_key(&self) -> Result<PublicKey, WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|_| WalletError::InvalidEcdsaPublicKey)?;

        Ok(public_key)
    }

    pub fn get_address(&self, network: Chains) -> Result<String, WalletError> {
        self.addresses
            .get(&network)
            .cloned()
            .ok_or_else(|| WalletError::MissingAddress)
    }

    pub fn get_sns_address(&self, token: String) -> Result<String, WalletError> {
        self.get_address(Chains::SNS(token))
    }

    pub fn get_eth_address(&self) -> Result<String, WalletError> {
        let ecdsa = self.ecdsa()?;

        let pub_key = secp256k1::PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .serialize_uncompressed();

        let keccak256 = raw_keccak256(&pub_key[1..]);

        let keccak256_hex = keccak256.to_hex_string();

        let address: String = "0x".to_owned() + &keccak256_hex[24..];

        Ok(address)
    }

    pub fn get_btc_address(&self, btc_network: BtcNetwork) -> Result<Address, WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|e| WalletError::GenerateError(e.to_string()))?;

        let address = Address::p2pkh(&public_key, btc_network.into());

        Ok(address)
    }

    pub fn set_ecdsa(&mut self, ecdsa: Vec<u8>) -> Result<(), WalletError> {
        if ecdsa.len() != 33 {
            return Err(WalletError::InvalidEcdsaPublicKey);
        }

        if self.is_ecdsa_set() {
            return Err(WalletError::EcdsaPublicKeyAlreadySet);
        }

        let ecdsa = PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .to_bytes();

        self.ecdsa = Some(ecdsa);

        self.generate_eth_address(0)?;

        self.generate_btc_address(BtcNetwork::Mainnet)?;

        Ok(())
    }

    pub fn generate_address(&mut self, network: Chains) -> Result<(), WalletError> {
        match network {
            Chains::EVM(chain) => self.generate_eth_address(chain),
            Chains::SNS(token) => self.generate_sns_address(token),
            Chains::BTC(btc_network) => self.generate_btc_address(btc_network),
            Chains::ICP => Ok(()),
        }
    }

    pub fn generate_sns_address(&mut self, token: String) -> Result<(), WalletError> {
        let address = self.identifier.to_string();

        self.addresses.insert(Chains::SNS(token), address.clone());

        Ok(())
    }

    pub fn generate_eth_address(&mut self, chain: u64) -> Result<(), WalletError> {
        let ecdsa = self.ecdsa()?;

        let pub_key = secp256k1::PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .serialize_uncompressed();

        let keccak256 = raw_keccak256(&pub_key[1..]);

        let keccak256_hex = keccak256.to_hex_string();

        let address: String = "0x".to_owned() + &keccak256_hex[24..];

        self.addresses.insert(Chains::EVM(chain), address.clone());

        Ok(())
    }

    pub fn generate_btc_address(&mut self, btc_network: BtcNetwork) -> Result<(), WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|e| WalletError::GenerateError(e.to_string()))?;

        let address = Address::p2pkh(&public_key, btc_network.into()).to_string();

        self.addresses
            .insert(Chains::BTC(btc_network), address.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use b3_helper_lib::types::CanisterId;

    use super::*;

    #[test]
    fn test_generate_address1() {
        let subaccount = Subaccount([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let mut public_keys = Keys {
            identifier: subaccount.account_identifier(owner),
            ecdsa: None,
            addresses: AddressMap::new(),
        };

        let ecdsa = vec![
            3, 94, 114, 171, 76, 217, 209, 126, 120, 169, 209, 205, 226, 55, 21, 238, 204, 199,
            153, 192, 65, 30, 59, 177, 153, 39, 80, 76, 185, 200, 51, 255, 218,
        ];

        public_keys.set_ecdsa(ecdsa).unwrap();

        let icp_address = public_keys.identifier().to_string();

        assert_eq!(
            icp_address,
            "368aef23bd675b853b05526e0d6fc91fb6cf20d111c51105a041eedc12b91111"
        );

        println!("icp_address: {}", icp_address);

        public_keys.generate_eth_address(1).unwrap();

        let eth_address = public_keys.get_eth_address().unwrap();

        assert_eq!(eth_address, "0x7e87f653ec3e9c6cde261e0e2e3e9c14bbe86802");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        public_keys
            .generate_btc_address(BtcNetwork::Regtest)
            .unwrap();

        let btc_address = public_keys
            .get_address(Chains::BTC(BtcNetwork::Regtest))
            .unwrap();

        assert_eq!(btc_address, "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");

        println!("btc_address: {}", btc_address);

        public_keys
            .generate_btc_address(BtcNetwork::Mainnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Chains::BTC(BtcNetwork::Mainnet))
            .unwrap();

        assert_eq!(btc_address, "1MnmPQSjKMGaruN9vbc6NFWizXGz6SgpdC");

        public_keys
            .generate_btc_address(BtcNetwork::Testnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Chains::BTC(BtcNetwork::Testnet))
            .unwrap();

        assert_eq!(btc_address, "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");

        assert_eq!(btc_address.len(), 34);
    }

    #[test]
    fn test_generate_address2() {
        let subaccount = Subaccount([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("7uoyg-piaaa-aaaap-abbzq-cai").unwrap();

        let mut public_keys = Keys {
            identifier: subaccount.account_identifier(owner),
            ecdsa: None,
            addresses: AddressMap::new(),
        };

        let ecdsa: EcdsaPublicKey = vec![
            2, 50, 207, 109, 252, 71, 63, 226, 215, 137, 36, 108, 105, 51, 80, 125, 193, 121, 151,
            101, 197, 65, 64, 240, 22, 142, 247, 130, 65, 210, 0, 176, 231,
        ];

        let identifier = public_keys.identifier();

        let expected_identifier = AccountIdentifier::from(vec![
            58, 236, 90, 93, 136, 79, 92, 97, 73, 20, 45, 129, 49, 134, 70, 254, 51, 92, 198, 124,
            199, 3, 100, 84, 204, 249, 218, 50, 237, 120, 84, 113,
        ]);

        println!("identifier: {:?}", identifier);

        assert_eq!(identifier.to_string(), expected_identifier.to_string());

        public_keys.set_ecdsa(ecdsa).unwrap();

        let icp_address = public_keys.identifier().to_string();

        assert_eq!(
            icp_address,
            "3aec5a5d884f5c6149142d81318646fe335cc67cc7036454ccf9da32ed785471"
        );

        println!("icp_address: {}", icp_address);

        public_keys.generate_eth_address(1).unwrap();

        let eth_address = public_keys.get_address(Chains::EVM(1)).unwrap();

        assert_eq!(eth_address, "0xd0406029f0703f6c04176c16451ce3a324f723c0");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        public_keys
            .generate_btc_address(BtcNetwork::Mainnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Chains::BTC(BtcNetwork::Mainnet))
            .unwrap();

        assert_eq!(btc_address, "1L2NEvApixneBNULQzcC5qysuWXrCNDhhr");

        println!("btc_address: {}", btc_address);

        assert_eq!(btc_address.len(), 34);
    }

    #[test]
    fn test_generate_address3() {
        let subaccount = Subaccount([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("us6ps-xaaaa-aaaap-aa5za-cai").unwrap();

        let identifier = subaccount.account_identifier(owner);

        println!("identifier: {}", identifier.to_string());

        let expected_identifier = AccountIdentifier::from(vec![
            222, 100, 102, 206, 2, 155, 237, 69, 41, 46, 16, 39, 70, 114, 4, 61, 28, 30, 202, 20,
            83, 162, 177, 143, 70, 7, 126, 129, 179, 65, 189, 8,
        ]);

        assert_eq!(identifier, expected_identifier);

        let mut public_keys = Keys {
            identifier,
            ecdsa: None,
            addresses: AddressMap::new(),
        };

        let ecdsa: EcdsaPublicKey = vec![
            2, 62, 198, 199, 5, 110, 183, 99, 191, 29, 195, 92, 118, 155, 254, 120, 1, 161, 5, 168,
            26, 182, 33, 68, 123, 186, 216, 216, 41, 136, 9, 40, 38,
        ];

        public_keys.set_ecdsa(ecdsa).unwrap();

        let icp_address = public_keys.identifier().to_string();

        assert_eq!(
            icp_address,
            "de6466ce029bed45292e10274672043d1c1eca1453a2b18f46077e81b341bd08"
        );

        println!("icp_address: {}", icp_address);

        public_keys.generate_eth_address(1).unwrap();

        let eth_address = public_keys.get_address(Chains::EVM(1)).unwrap();

        assert_eq!(eth_address, "0x82f3031c7bd2cd7e5c6d4d83584656b873304502");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        public_keys
            .generate_btc_address(BtcNetwork::Mainnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Chains::BTC(BtcNetwork::Mainnet))
            .unwrap();

        assert_eq!(btc_address, "18P7514xYnwxHcWuc96Ae7dPqhX2syiS2m");

        println!("btc_address: {}", btc_address);

        assert_eq!(btc_address.len(), 34);
    }
}

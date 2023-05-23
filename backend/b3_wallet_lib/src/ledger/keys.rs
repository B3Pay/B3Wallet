use crate::error::WalletError;
use b3_helper::sha3_sha256;
use b3_helper::types::{AccountIdentifier, Subaccount};

use bitcoin::{Address, PublicKey};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::HashMap;

use super::network::BtcNetwork;
use super::subaccount::SubaccountTrait;
use super::{
    network::Network,
    types::{AddressMap, EcdsaPublicKey},
};

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
        let canister = ic_cdk::id();

        let identifier = subaccount.account_identifier(canister);

        let mut addresses = AddressMap::new();

        addresses.insert(Network::ICP, identifier.to_string());

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

    pub fn get_address(&self, network: Network) -> Result<String, WalletError> {
        match network {
            Network::EVM(chain) => self.get_eth_address(chain),
            Network::SNS(token) => self.get_sns_address(token),
            Network::BTC(btc_network) => self.get_btc_address(btc_network),
            Network::ICP => Ok(self.identifier.to_string()),
        }
    }

    pub fn get_sns_address(&self, token: String) -> Result<String, WalletError> {
        match self.addresses.get(&Network::SNS(token)) {
            Some(address) => Ok(address.clone()),
            None => Err(WalletError::MissingAddress),
        }
    }

    pub fn get_eth_address(&self, chain: u64) -> Result<String, WalletError> {
        match self.addresses.get(&Network::EVM(chain)) {
            Some(address) => Ok(address.clone()),
            None => Err(WalletError::MissingAddress),
        }
    }

    pub fn get_btc_address(&self, btc_network: BtcNetwork) -> Result<String, WalletError> {
        match self.addresses.get(&Network::BTC(btc_network)) {
            Some(address) => Ok(address.clone()),
            None => Err(WalletError::MissingAddress),
        }
    }

    pub fn set_ecdsa(&mut self, ecdsa: Vec<u8>) -> Result<(), WalletError> {
        if self.is_ecdsa_set() {
            return Err(WalletError::EcdsaPublicKeyAlreadySet);
        }

        if ecdsa.len() != 33 {
            return Err(WalletError::InvalidEcdsaPublicKey);
        }

        let ecdsa = PublicKey::from_slice(&ecdsa)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .to_bytes();

        self.ecdsa = Some(ecdsa);

        self.generate_eth_address(0)?;

        self.generate_btc_address(BtcNetwork::Mainnet)?;

        Ok(())
    }

    pub fn generate_address(&mut self, network: Network) -> Result<(), WalletError> {
        match network {
            Network::EVM(chain) => self.generate_eth_address(chain),
            Network::SNS(token) => self.generate_sns_address(token),
            Network::BTC(btc_network) => self.generate_btc_address(btc_network),
            Network::ICP => Ok(()),
        }
    }

    pub fn generate_sns_address(&mut self, token: String) -> Result<(), WalletError> {
        let address = self.identifier.to_string();

        self.addresses.insert(Network::SNS(token), address.clone());

        Ok(())
    }

    pub fn generate_eth_address(&mut self, chain: u64) -> Result<(), WalletError> {
        let ecdsa = self.ecdsa()?;

        let pub_key_arr: [u8; 33] = ecdsa[..].try_into().unwrap();

        let pub_key = PublicKey::from_slice(&pub_key_arr)
            .map_err(|e| WalletError::GenerateError(e.to_string()))?
            .to_bytes();

        let keccak256 = sha3_sha256(&pub_key[1..]);

        let keccak256_hex = keccak256
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        let address: String = "0x".to_owned() + &keccak256_hex[24..];

        self.addresses.insert(Network::EVM(chain), address.clone());

        Ok(())
    }

    pub fn generate_btc_address(&mut self, btc_network: BtcNetwork) -> Result<(), WalletError> {
        let ecdsa = self.ecdsa()?;

        let public_key =
            PublicKey::from_slice(&ecdsa).map_err(|e| WalletError::GenerateError(e.to_string()))?;

        let address = Address::p2pkh(&public_key, btc_network.into()).to_string();

        self.addresses
            .insert(Network::BTC(btc_network), address.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use b3_helper::types::CanisterId;

    use super::*;

    #[test]
    fn test_generate_address1() {
        let subaccount = Subaccount([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let mut public_keys = Keys {
            identifier: AccountIdentifier::new(owner, subaccount),
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
            "e237f0b05cb1dc302b3dae980ca9efab50dc7e2b165f7ac971c20a25d50b5f68"
        );

        println!("icp_address: {}", icp_address);

        public_keys.generate_eth_address(1).unwrap();

        let eth_address = public_keys.addresses.get(&Network::EVM(1)).unwrap();

        assert_eq!(eth_address, "0x004014307c1bfb1dec4eec9661cea77b5826d01d");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        public_keys
            .generate_btc_address(BtcNetwork::Regtest)
            .unwrap();

        let btc_address = public_keys
            .get_address(Network::BTC(BtcNetwork::Regtest))
            .unwrap();

        assert_eq!(btc_address, "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");

        println!("btc_address: {}", btc_address);

        public_keys
            .generate_btc_address(BtcNetwork::Mainnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Network::BTC(BtcNetwork::Mainnet))
            .unwrap();

        assert_eq!(btc_address, "1MnmPQSjKMGaruN9vbc6NFWizXGz6SgpdC");

        public_keys
            .generate_btc_address(BtcNetwork::Testnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Network::BTC(BtcNetwork::Testnet))
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
            identifier: AccountIdentifier::new(owner, subaccount),
            ecdsa: None,
            addresses: AddressMap::new(),
        };

        let ecdsa: EcdsaPublicKey = vec![
            2, 50, 207, 109, 252, 71, 63, 226, 215, 137, 36, 108, 105, 51, 80, 125, 193, 121, 151,
            101, 197, 65, 64, 240, 22, 142, 247, 130, 65, 210, 0, 176, 231,
        ];

        let identifier = public_keys.identifier();

        let expected_identifier = AccountIdentifier::from(vec![
            89, 200, 125, 160, 1, 190, 8, 190, 208, 172, 35, 20, 163, 214, 155, 189, 28, 113, 45,
            177, 78, 207, 45, 150, 87, 215, 96, 119, 136, 171, 118, 18,
        ]);

        println!("identifier: {}", identifier.to_string());

        assert_eq!(identifier.to_string(), expected_identifier.to_string());

        public_keys.set_ecdsa(ecdsa).unwrap();

        let icp_address = public_keys.identifier().to_string();

        assert_eq!(
            icp_address,
            "59c87da001be08bed0ac2314a3d69bbd1c712db14ecf2d9657d7607788ab7612"
        );

        println!("icp_address: {}", icp_address);

        public_keys.generate_eth_address(1).unwrap();

        let eth_address = public_keys.get_address(Network::EVM(1)).unwrap();

        assert_eq!(eth_address, "0x9eea1bf5d05e30b900db4471c3839e68417fbcc5");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        public_keys
            .generate_btc_address(BtcNetwork::Mainnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Network::BTC(BtcNetwork::Mainnet))
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

        let identifier = AccountIdentifier::new(owner, subaccount);

        println!("identifier: {}", identifier.to_string());

        let expected_identifier = AccountIdentifier::from(vec![
            140, 144, 174, 128, 153, 211, 171, 43, 103, 68, 188, 143, 155, 91, 236, 172, 118, 117,
            50, 203, 132, 3, 4, 30, 101, 124, 179, 110, 127, 51, 62, 0,
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
            "8c90ae8099d3ab2b6744bc8f9b5becac767532cb8403041e657cb36e7f333e00"
        );

        println!("icp_address: {}", icp_address);

        public_keys.generate_eth_address(1).unwrap();

        let eth_address = public_keys.get_address(Network::EVM(1)).unwrap();

        assert_eq!(eth_address, "0x0dd99dc1a94a3ca699f6bdbd87c7ff07a31cacb6");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        public_keys
            .generate_btc_address(BtcNetwork::Mainnet)
            .unwrap();

        let btc_address = public_keys
            .get_address(Network::BTC(BtcNetwork::Mainnet))
            .unwrap();

        assert_eq!(btc_address, "18P7514xYnwxHcWuc96Ae7dPqhX2syiS2m");

        println!("btc_address: {}", btc_address);

        assert_eq!(btc_address.len(), 34);
    }
}

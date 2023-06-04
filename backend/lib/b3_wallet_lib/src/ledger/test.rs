#[cfg(test)]
mod tests {
    use crate::ledger::{
        btc::network::BtcNetwork,
        types::{Chain, ChainMap, ChainTrait, ChainType, Ledger},
    };
    use b3_helper_lib::types::{AccountIdentifier, CanisterId, Subaccount};

    use crate::ledger::types::EcdsaPublicKey;

    #[test]
    fn test_generate_address1() {
        let subaccount = Subaccount([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let identifier = AccountIdentifier::new(owner, subaccount.clone());

        let mut chains = ChainMap::new();

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        chains.insert(ChainType::ICP, icp_chain);

        let mut ledger = Ledger {
            ecdsa: None,
            subaccount,
            chains,
        };

        let ecdsa = vec![
            3, 94, 114, 171, 76, 217, 209, 126, 120, 169, 209, 205, 226, 55, 21, 238, 204, 199,
            153, 192, 65, 30, 59, 177, 153, 39, 80, 76, 185, 200, 51, 255, 218,
        ];

        ledger.set_ecdsa(ecdsa).unwrap();

        assert_eq!(
            identifier.to_string(),
            "368aef23bd675b853b05526e0d6fc91fb6cf20d111c51105a041eedc12b91111"
        );

        println!("identifier: {}", identifier);

        ledger.generate_eth_address(1).unwrap();

        let eth_address = ledger.eth_address().unwrap();

        assert_eq!(eth_address, "0x7e87f653ec3e9c6cde261e0e2e3e9c14bbe86802");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        ledger.generate_btc_address(BtcNetwork::Regtest).unwrap();

        let btc_address = ledger.chain(ChainType::BTC(BtcNetwork::Regtest)).unwrap();

        assert_eq!(btc_address.address(), "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");

        println!("btc_address: {}", btc_address.address());

        ledger.generate_btc_address(BtcNetwork::Mainnet).unwrap();

        let btc_address = ledger.chain(ChainType::BTC(BtcNetwork::Mainnet)).unwrap();

        assert_eq!(btc_address.address(), "1MnmPQSjKMGaruN9vbc6NFWizXGz6SgpdC");

        ledger.generate_btc_address(BtcNetwork::Testnet).unwrap();

        let btc_address = ledger.chain(ChainType::BTC(BtcNetwork::Testnet)).unwrap();

        assert_eq!(btc_address.address(), "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");

        assert_eq!(btc_address.address().len(), 34);
    }

    #[test]
    fn test_generate_address2() {
        let subaccount = Subaccount([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let identifier = AccountIdentifier::new(owner, subaccount.clone());

        let expected_identifier = AccountIdentifier::from(vec![
            146, 231, 37, 157, 114, 49, 157, 239, 199, 132, 229, 111, 180, 128, 68, 147, 19, 27,
            21, 176, 125, 49, 244, 123, 149, 241, 38, 235, 86, 180, 38, 113,
        ]);

        println!("identifier: {:?}", identifier);

        let mut chains = ChainMap::new();

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        chains.insert(ChainType::ICP, icp_chain);

        let mut ledger = Ledger {
            ecdsa: None,
            subaccount,
            chains,
        };

        let ecdsa: EcdsaPublicKey = vec![
            2, 50, 207, 109, 252, 71, 63, 226, 215, 137, 36, 108, 105, 51, 80, 125, 193, 121, 151,
            101, 197, 65, 64, 240, 22, 142, 247, 130, 65, 210, 0, 176, 231,
        ];

        assert_eq!(identifier.to_string(), expected_identifier.to_string());

        ledger.set_ecdsa(ecdsa).unwrap();

        let icp_address = ledger.icp_chain().unwrap();

        assert_eq!(
            icp_address.address(),
            "92e7259d72319defc784e56fb4804493131b15b07d31f47b95f126eb56b42671"
        );

        println!("icp_address: {}", icp_address.address());

        ledger.generate_eth_address(1).unwrap();

        let eth_address = ledger.chain(ChainType::EVM(1)).unwrap();

        assert_eq!(
            eth_address.address(),
            "0xd0406029f0703f6c04176c16451ce3a324f723c0"
        );

        println!("eth_address: {}", eth_address.address());

        assert_eq!(eth_address.address().len(), 42);

        ledger.generate_btc_address(BtcNetwork::Mainnet).unwrap();

        let btc_address = ledger.chain(ChainType::BTC(BtcNetwork::Mainnet)).unwrap();

        assert_eq!(btc_address.address(), "1L2NEvApixneBNULQzcC5qysuWXrCNDhhr");

        println!("btc_address: {}", btc_address.address());

        assert_eq!(btc_address.address().len(), 34);
    }

    #[test]
    fn test_generate_address3() {
        let subaccount = Subaccount([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let identifier = subaccount.account_identifier(owner);

        println!("identifier: {}", identifier.to_string());

        let expected_identifier = AccountIdentifier::from(vec![
            146, 231, 37, 157, 114, 49, 157, 239, 199, 132, 229, 111, 180, 128, 68, 147, 19, 27,
            21, 176, 125, 49, 244, 123, 149, 241, 38, 235, 86, 180, 38, 113,
        ]);

        assert_eq!(identifier, expected_identifier);

        let mut chains = ChainMap::new();

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        chains.insert(ChainType::ICP, icp_chain);

        let mut ledger = Ledger {
            ecdsa: None,
            subaccount,
            chains,
        };

        let ecdsa: EcdsaPublicKey = vec![
            2, 62, 198, 199, 5, 110, 183, 99, 191, 29, 195, 92, 118, 155, 254, 120, 1, 161, 5, 168,
            26, 182, 33, 68, 123, 186, 216, 216, 41, 136, 9, 40, 38,
        ];

        ledger.set_ecdsa(ecdsa).unwrap();

        let icp_address = ledger.icp_chain().unwrap();

        assert_eq!(
            icp_address.address(),
            "92e7259d72319defc784e56fb4804493131b15b07d31f47b95f126eb56b42671"
        );

        println!("icp_address: {}", icp_address.address());

        ledger.generate_eth_address(1).unwrap();

        let eth_address = ledger.chain(ChainType::EVM(1)).unwrap();

        assert_eq!(
            eth_address.address(),
            "0x82f3031c7bd2cd7e5c6d4d83584656b873304502"
        );

        println!("eth_address: {}", eth_address.address());

        assert_eq!(eth_address.address().len(), 42);

        ledger.generate_btc_address(BtcNetwork::Mainnet).unwrap();

        let btc_address = ledger.chain(ChainType::BTC(BtcNetwork::Mainnet)).unwrap();

        assert_eq!(btc_address.address(), "18P7514xYnwxHcWuc96Ae7dPqhX2syiS2m");

        println!("btc_address: {}", btc_address.address());

        assert_eq!(btc_address.address().len(), 34);
    }
}

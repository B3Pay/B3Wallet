#[cfg(test)]
mod tests {
    use crate::ledger::{
        btc::{
            address::{
                network_and_public_key_to_p2pkh, network_and_public_key_to_p2wpkh, BitcoinAddress,
            },
            network::BitcoinNetwork,
        },
        chain::{Chain, ChainTrait},
        ledger::Ledger,
        types::{ChainEnum, ChainMap},
    };
    use b3_utils::{ledger::AccountIdentifier, mocks::id_mock, types::CanisterId, Subaccount};
    use libsecp256k1::PublicKey;

    #[test]
    fn test_generate_address1() {
        let subaccount = Subaccount([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let identifier = AccountIdentifier::new(owner, Some(subaccount.clone()));

        let mut chains = ChainMap::new();

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        chains.insert(ChainEnum::ICP, icp_chain);

        let mut ledger = Ledger {
            public_key: None,
            subaccount: subaccount.clone(),
            chains,
        };

        let ecdsa = [
            3, 94, 114, 171, 76, 217, 209, 126, 120, 169, 209, 205, 226, 55, 21, 238, 204, 199,
            153, 192, 65, 30, 59, 177, 153, 39, 80, 76, 185, 200, 51, 255, 218,
        ];

        ledger.set_ecdsa_public_key(ecdsa.to_vec()).unwrap();

        assert_eq!(
            identifier.to_string(),
            "368aef23bd675b853b05526e0d6fc91fb6cf20d111c51105a041eedc12b91111"
        );

        println!("identifier: {}", identifier);

        let address = ledger.eth_address().unwrap();

        let eth = Chain::new_evm_chain(1, address);

        ledger.insert_chain(ChainEnum::EVM(1), eth);

        let eth_address = ledger.chain(&ChainEnum::EVM(1)).unwrap().address();

        assert_eq!(eth_address, "0x7e87f653ec3e9c6cde261e0e2e3e9c14bbe86802");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);
        let public_key = PublicKey::parse_compressed(&ecdsa).unwrap();

        let chain =
            Chain::new_btc_chain(BitcoinNetwork::Regtest, subaccount.clone(), public_key).unwrap();

        ledger.insert_chain(ChainEnum::BTC(BitcoinNetwork::Regtest), chain.clone());

        let btc_chain = ledger
            .chain(&ChainEnum::BTC(BitcoinNetwork::Regtest))
            .unwrap();

        let btc_p2pkh_add = network_and_public_key_to_p2pkh(
            BitcoinNetwork::Regtest,
            public_key.serialize_compressed().as_slice(),
        );

        assert_eq!(btc_p2pkh_add, "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");
        assert_eq!(
            btc_chain.address(),
            "bcrt1qus9yjf6s8cv0pyqfp2lxgp57xc356uxzcglff6"
        );

        assert_eq!(chain.address(), btc_chain.address());

        println!("regtest address: {}", btc_chain.address());

        let chain = Chain::new_btc_chain(
            BitcoinNetwork::Mainnet,
            subaccount.clone(),
            PublicKey::parse_compressed(&ecdsa).unwrap(),
        )
        .unwrap();

        ledger.insert_chain(ChainEnum::BTC(BitcoinNetwork::Mainnet), chain);

        let btc_address = ledger
            .chain(&ChainEnum::BTC(BitcoinNetwork::Mainnet))
            .unwrap();

        let btc_p2pkh_add = network_and_public_key_to_p2pkh(
            BitcoinNetwork::Mainnet,
            public_key.serialize_compressed().as_slice(),
        );

        assert_eq!(btc_p2pkh_add, "1MnmPQSjKMGaruN9vbc6NFWizXGz6SgpdC");

        assert_eq!(
            btc_address.address(),
            "bc1qus9yjf6s8cv0pyqfp2lxgp57xc356uxzs8ah9q"
        );

        let chain = Chain::new_btc_chain(
            BitcoinNetwork::Testnet,
            subaccount.clone(),
            PublicKey::parse_compressed(&ecdsa).unwrap(),
        )
        .unwrap();

        ledger.insert_chain(ChainEnum::BTC(BitcoinNetwork::Testnet), chain);

        let btc_address = ledger
            .chain(&ChainEnum::BTC(BitcoinNetwork::Testnet))
            .unwrap();

        let btc_p2pkh_add = network_and_public_key_to_p2pkh(
            BitcoinNetwork::Regtest,
            public_key.serialize_compressed().as_slice(),
        );

        assert_eq!(btc_p2pkh_add, "n2JigTXi8Nhqe1qmeAaUCAj3rWsgxRzMe3");
        assert_eq!(
            btc_address.address(),
            "tb1qus9yjf6s8cv0pyqfp2lxgp57xc356uxz6pxy7n"
        );

        assert_eq!(btc_p2pkh_add.len(), 34);
        assert_eq!(btc_address.address().len(), 42);
    }

    #[test]
    fn test_generate_address2() {
        let subaccount = Subaccount([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let identifier = AccountIdentifier::new(id_mock(), Some(subaccount.clone()));

        let expected_identifier = AccountIdentifier::from_slice(
            [
                45, 14, 137, 127, 126, 134, 45, 43, 87, 217, 188, 158, 165, 198, 95, 154, 36, 172,
                108, 7, 69, 117, 244, 120, 152, 49, 75, 141, 108, 176, 146, 157,
            ]
            .as_slice(),
        )
        .unwrap();

        println!("identifier: {:?}", identifier);

        assert_eq!(identifier.to_string(), expected_identifier.to_string());

        let mut chains = ChainMap::new();

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        chains.insert(ChainEnum::ICP, icp_chain);

        let mut ledger = Ledger {
            public_key: None,
            subaccount: subaccount.clone(),
            chains,
        };

        let ecdsa = [
            2, 50, 207, 109, 252, 71, 63, 226, 215, 137, 36, 108, 105, 51, 80, 125, 193, 121, 151,
            101, 197, 65, 64, 240, 22, 142, 247, 130, 65, 210, 0, 176, 231,
        ];

        ledger.set_ecdsa_public_key(ecdsa.to_vec()).unwrap();

        let icp_address = ledger.chain(&ChainEnum::ICP).unwrap();

        assert_eq!(
            icp_address.address(),
            "2d0e897f7e862d2b57d9bc9ea5c65f9a24ac6c074575f47898314b8d6cb0929d"
        );

        println!("icp_address: {}", icp_address.address());

        let eth_address = ledger.eth_address().unwrap();

        assert_eq!(eth_address, "0xd0406029f0703f6c04176c16451ce3a324f723c0");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        let public_key = PublicKey::parse_compressed(&ecdsa).unwrap();

        let chain = Chain::new_btc_chain(BitcoinNetwork::Mainnet, subaccount, public_key).unwrap();

        ledger.insert_chain(ChainEnum::BTC(BitcoinNetwork::Mainnet), chain.clone());

        let ledger_chain = ledger
            .chain(&ChainEnum::BTC(BitcoinNetwork::Mainnet))
            .unwrap();

        let btc_p2pkh_add = network_and_public_key_to_p2pkh(
            BitcoinNetwork::Mainnet,
            public_key.serialize_compressed().as_slice(),
        );

        assert_eq!(btc_p2pkh_add, "1L2NEvApixneBNULQzcC5qysuWXrCNDhhr");
        assert_eq!(
            chain.address(),
            "bc1q6zkgjwu5w7lzc8sgja6uk24ex7zegwpwq0pet9"
        );

        assert_eq!(btc_p2pkh_add.len(), 34);
        assert_eq!(chain.address().len(), 42);

        assert_eq!(ledger_chain.address(), chain.address());

        println!("btc_address: {}", chain.address());

        let btc_add = BitcoinAddress::parse(&chain.address(), BitcoinNetwork::Mainnet).unwrap();

        println!(
            "bitcoin_adress: {:?}",
            btc_add.display(BitcoinNetwork::Mainnet)
        );

        assert_eq!(btc_add.display(BitcoinNetwork::Mainnet), chain.address());

        let bitcoin_adress = network_and_public_key_to_p2wpkh(BitcoinNetwork::Mainnet, &ecdsa);

        println!("bitcoin_adress: {}", bitcoin_adress);
    }

    #[test]
    fn test_generate_address3() {
        let subaccount = Subaccount([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let owner = CanisterId::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

        let identifier = AccountIdentifier::new(owner, Some(subaccount.clone()));

        println!("identifier: {}", identifier.to_string());

        let expected_identifier = AccountIdentifier::from_slice(
            [
                146, 231, 37, 157, 114, 49, 157, 239, 199, 132, 229, 111, 180, 128, 68, 147, 19,
                27, 21, 176, 125, 49, 244, 123, 149, 241, 38, 235, 86, 180, 38, 113,
            ]
            .as_slice(),
        )
        .unwrap();

        assert_eq!(identifier, expected_identifier);

        let mut chains = ChainMap::new();

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        chains.insert(ChainEnum::ICP, icp_chain);

        let mut ledger = Ledger {
            public_key: None,
            subaccount: subaccount.clone(),
            chains,
        };

        let ecdsa = [
            2, 62, 198, 199, 5, 110, 183, 99, 191, 29, 195, 92, 118, 155, 254, 120, 1, 161, 5, 168,
            26, 182, 33, 68, 123, 186, 216, 216, 41, 136, 9, 40, 38,
        ];

        ledger.set_ecdsa_public_key(ecdsa.to_vec()).unwrap();

        let icp_address = ledger.chain(&ChainEnum::ICP).unwrap();

        assert_eq!(
            icp_address.address(),
            "2d0e897f7e862d2b57d9bc9ea5c65f9a24ac6c074575f47898314b8d6cb0929d"
        );

        println!("icp_address: {}", icp_address.address());

        let eth_address = ledger.eth_address().unwrap();

        assert_eq!(eth_address, "0x82f3031c7bd2cd7e5c6d4d83584656b873304502");

        println!("eth_address: {}", eth_address);

        assert_eq!(eth_address.len(), 42);

        let chain = Chain::new_btc_chain(
            BitcoinNetwork::Testnet,
            subaccount,
            PublicKey::parse_compressed(&ecdsa).unwrap(),
        )
        .unwrap();

        ledger.insert_chain(ChainEnum::BTC(BitcoinNetwork::Testnet), chain);

        let btc_address = ledger
            .chain(&ChainEnum::BTC(BitcoinNetwork::Testnet))
            .unwrap()
            .address();

        let btc_p2pkh_add = network_and_public_key_to_p2pkh(
            BitcoinNetwork::Testnet,
            PublicKey::parse_compressed(&ecdsa)
                .unwrap()
                .serialize_compressed()
                .as_slice(),
        );

        assert_eq!(btc_p2pkh_add, "mnu4N49wMpPD4izXKi4YU2qihh7jnmNtjt");
        assert_eq!(btc_address, "tb1q2rm3jzqunjhcw0nsjp8ttjtsqdpeluypsy0p8h");

        println!("testnet address: {}", btc_address);

        assert_eq!(btc_p2pkh_add.len(), 34);
        assert_eq!(btc_address.len(), 42);
    }
}

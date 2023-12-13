use std::collections::HashMap;
use std::str::FromStr;

use crate::ledger::btc::error::BitcoinError;

use super::network::BitcoinNetwork;
use super::types::{OutPoint, Satoshi, Utxo};
use super::utxos::BitcoinUtxos;
use super::{address::BitcoinAddress, signature::EncodedSignature, tx};
use crate::ledger::btc::utils::tx_vsize_estimate;
use bitcoin::address::Payload;
use bitcoin::consensus::{deserialize, serialize};
use bitcoin::hashes::Hash;
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::{Amount, Network, Transaction, WitnessProgram};
use bitcoin::{Sequence, WitnessVersion};

use proptest::proptest;
use proptest::{
    array::uniform20,
    array::uniform32,
    collection::vec as pvec,
    option,
    prelude::{any, Strategy},
};
use proptest::{prop_assert, prop_assert_eq, prop_assume, prop_oneof};
use serde_bytes::ByteBuf;

fn dummy_utxo_from_value(v: u64) -> Utxo {
    let mut bytes = [0u8; 32];
    bytes[0..8].copy_from_slice(&v.to_be_bytes());
    Utxo {
        outpoint: OutPoint {
            txid: bytes.into(),
            vout: 0,
        },
        value: v,
        height: 0,
    }
}

fn address_to_script_pubkey(address: &BitcoinAddress) -> bitcoin::ScriptBuf {
    let address_string = address.display(BitcoinNetwork::Mainnet);
    let btc_address = bitcoin::Address::from_str(&address_string).unwrap();
    btc_address
        .require_network(Network::Bitcoin)
        .unwrap()
        .script_pubkey()
}

fn btc_network_to_network(network: BitcoinNetwork) -> Network {
    match network {
        BitcoinNetwork::Mainnet => Network::Bitcoin,
        BitcoinNetwork::Testnet => Network::Testnet,
        BitcoinNetwork::Regtest => Network::Regtest,
    }
}

fn address_to_btc_address(address: &BitcoinAddress, network: Network) -> bitcoin::Address {
    match address {
        BitcoinAddress::P2wpkhV0(pkhash) => bitcoin::Address::new(
            network,
            Payload::WitnessProgram(
                WitnessProgram::new(WitnessVersion::V0, pkhash.to_vec())
                    .expect("failed to create a witness program"),
            ),
        ),
        BitcoinAddress::P2wshV0(script_hash) => bitcoin::Address::new(
            network,
            Payload::WitnessProgram(
                WitnessProgram::new(WitnessVersion::V0, script_hash.to_vec())
                    .expect("failed to create a witness program"),
            ),
        ),
        BitcoinAddress::P2pkh(pkhash) => bitcoin::Address::new(
            network,
            Payload::PubkeyHash(bitcoin::PubkeyHash::from_raw_hash(
                bitcoin::hashes::Hash::from_slice(pkhash).unwrap(),
            )),
        ),
        BitcoinAddress::P2sh(script_hash) => bitcoin::Address::new(
            network,
            Payload::ScriptHash(bitcoin::ScriptHash::from_raw_hash(
                bitcoin::hashes::Hash::from_slice(script_hash).unwrap(),
            )),
        ),
        BitcoinAddress::P2trV1(pkhash) => bitcoin::Address::new(
            network,
            Payload::WitnessProgram(
                WitnessProgram::new(WitnessVersion::V1, pkhash.to_vec()).unwrap(),
            ),
        ),
    }
}

fn as_txid(hash: &Vec<u8>) -> bitcoin::Txid {
    bitcoin::Txid::from_raw_hash(bitcoin::hashes::Hash::from_slice(hash).unwrap())
}

fn p2wpkh_script_code(pkhash: &[u8; 20]) -> bitcoin::script::ScriptBuf {
    use bitcoin::blockdata::{opcodes, script::Builder};

    Builder::new()
        .push_opcode(opcodes::all::OP_DUP)
        .push_opcode(opcodes::all::OP_HASH160)
        .push_slice(&pkhash)
        .push_opcode(opcodes::all::OP_EQUALVERIFY)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script()
}

fn unsigned_tx_to_bitcoin_tx(tx: &tx::UnsignedTransaction) -> bitcoin::Transaction {
    bitcoin::Transaction {
        version: Version::TWO,
        lock_time: bitcoin::absolute::LockTime::from_consensus(tx.lock_time),
        input: tx
            .inputs
            .iter()
            .map(|txin| bitcoin::TxIn {
                previous_output: bitcoin::OutPoint {
                    txid: as_txid(&txin.previous_output.txid),
                    vout: txin.previous_output.vout,
                },
                sequence: Sequence::from_consensus(txin.sequence),
                script_sig: bitcoin::Script::builder().into_script(),
                witness: bitcoin::Witness::default(),
            })
            .collect(),
        output: tx
            .outputs
            .iter()
            .map(|txout| bitcoin::TxOut {
                value: Amount::from_sat(txout.value),
                script_pubkey: address_to_script_pubkey(&txout.address),
            })
            .collect(),
    }
}

fn signed_tx_to_bitcoin_tx(tx: &tx::SignedTransaction) -> bitcoin::Transaction {
    bitcoin::Transaction {
        version: Version::TWO,
        lock_time: bitcoin::absolute::LockTime::from_consensus(tx.lock_time),
        input: tx
            .inputs
            .iter()
            .map(|txin| bitcoin::TxIn {
                previous_output: bitcoin::OutPoint {
                    txid: as_txid(&txin.previous_output.txid),
                    vout: txin.previous_output.vout,
                },
                sequence: Sequence::from_consensus(txin.sequence),
                script_sig: bitcoin::Script::builder().into_script(),
                witness: bitcoin::Witness::from(vec![
                    txin.signature.as_slice().to_vec(),
                    txin.pubkey.to_vec(),
                ]),
            })
            .collect(),
        output: tx
            .outputs
            .iter()
            .map(|txout| bitcoin::TxOut {
                value: Amount::from_sat(txout.value),
                script_pubkey: address_to_script_pubkey(&txout.address),
            })
            .collect(),
    }
}

#[test]
fn greedy_smoke_test() {
    let dummy_utxos: Vec<Utxo> = (1..10u64).map(dummy_utxo_from_value).collect();
    assert_eq!(dummy_utxos.len(), 9_usize);

    let utxos = BitcoinUtxos::from(dummy_utxos);

    assert_eq!(utxos.total_value(), 45_u64);

    let res = utxos.greedy(15);

    assert_eq!(res[0].value, 9_u64);
    assert_eq!(res[1].value, 6_u64);
}

#[test]
fn test_min_change_amount() {
    let mut available_utxos = BitcoinUtxos::new();
    available_utxos.insert(Utxo {
        outpoint: OutPoint {
            txid: [0; 32].into(),
            vout: 0,
        },
        value: 100_000,
        height: 10,
    });

    available_utxos.insert(Utxo {
        outpoint: OutPoint {
            txid: [1; 32].into(),
            vout: 1,
        },
        value: 100_000,
        height: 10,
    });

    let minter_addr = BitcoinAddress::P2wpkhV0([0; 20]);
    let out1_addr = BitcoinAddress::P2wpkhV0([1; 20]);
    let amount = 100_000;
    let fee_per_vbyte = 10000;

    let (tx, fee) = available_utxos
        .build_unsigned_transaction(&minter_addr, &out1_addr, amount, fee_per_vbyte)
        .expect("failed to build a transaction");

    let calc_fee = tx.fake_sign().vsize() as u64 * fee_per_vbyte / 1000;

    assert_eq!(fee, calc_fee);
    assert_eq!(tx.outputs[0].value, amount);
    assert_eq!(tx.outputs[1].value, amount - fee);
    assert_eq!(tx.outputs[2].value, 0);
}

#[test]
fn test_no_dust_outputs() {
    let mut available_utxos = BitcoinUtxos::new();
    available_utxos.insert(Utxo {
        outpoint: OutPoint {
            txid: [0; 32].into(),
            vout: 0,
        },
        value: 100_000,
        height: 10,
    });

    let minter_addr = BitcoinAddress::P2wpkhV0([0; 20]);
    let out1_addr = BitcoinAddress::P2wpkhV0([1; 20]);
    let fee_per_vbyte = 10000;

    assert_eq!(
        available_utxos.build_unsigned_transaction(&minter_addr, &out1_addr, 100, fee_per_vbyte,),
        Err(BitcoinError::DustOutput {
            address: minter_addr.display(BitcoinNetwork::Mainnet),
            amount: 100
        })
    );

    let fee_per_vbyte = 4000;

    assert_eq!(
        available_utxos.build_unsigned_transaction(&minter_addr, &out1_addr, 1000, fee_per_vbyte,),
        Err(BitcoinError::DustOutput {
            address: minter_addr.display(BitcoinNetwork::Mainnet),
            amount: 1000
        })
    );

    assert_eq!(available_utxos.len(), 1);
}

fn arb_amount() -> impl Strategy<Value = Satoshi> {
    1..10_000_000_000u64
}

fn arb_out_point() -> impl Strategy<Value = OutPoint> {
    (pvec(any::<u8>(), 32), any::<u32>()).prop_map(|(txid, vout)| OutPoint { txid, vout })
}

fn arb_unsigned_input(
    value: impl Strategy<Value = Satoshi>,
) -> impl Strategy<Value = tx::UnsignedInput> {
    (arb_out_point(), value, any::<u32>()).prop_map(|(previous_output, value, sequence)| {
        tx::UnsignedInput {
            previous_output,
            value,
            sequence,
        }
    })
}

fn arb_signed_input() -> impl Strategy<Value = tx::SignedInput> {
    (
        arb_out_point(),
        any::<u32>(),
        pvec(1u8..0xff, 64),
        pvec(any::<u8>(), 32),
    )
        .prop_map(
            |(previous_output, sequence, sec1, pubkey)| tx::SignedInput {
                previous_output,
                sequence,
                signature: EncodedSignature::from_sec1(&sec1),
                pubkey: ByteBuf::from(pubkey),
            },
        )
}

fn arb_address() -> impl Strategy<Value = BitcoinAddress> {
    prop_oneof![
        uniform20(any::<u8>()).prop_map(BitcoinAddress::P2wpkhV0),
        uniform32(any::<u8>()).prop_map(BitcoinAddress::P2wshV0),
        uniform32(any::<u8>()).prop_map(BitcoinAddress::P2trV1),
        uniform20(any::<u8>()).prop_map(BitcoinAddress::P2pkh),
        uniform20(any::<u8>()).prop_map(BitcoinAddress::P2sh),
    ]
}

fn arb_tx_out() -> impl Strategy<Value = tx::TxOut> {
    (arb_amount(), arb_address()).prop_map(|(value, address)| tx::TxOut { value, address })
}

fn arb_utxo(amount: impl Strategy<Value = Satoshi>) -> impl Strategy<Value = Utxo> {
    (amount, pvec(any::<u8>(), 32), 0..5u32).prop_map(|(value, txid, vout)| Utxo {
        outpoint: OutPoint { txid, vout },
        value,
        height: 0,
    })
}

proptest! {
    #[test]
    fn greedy_solution_properties(
        values in pvec(1u64..1_000_000_000, 1..10),
        target in 1u64..1_000_000_000,
    ) {
        let dummy_utxos: Vec<Utxo> = values
            .into_iter()
            .map(dummy_utxo_from_value)
            .collect();

        let mut utxos = BitcoinUtxos::from(dummy_utxos.clone());

        let total = dummy_utxos.iter().map(|u| u.value).sum::<u64>();

        if total < target {
            utxos.insert(dummy_utxo_from_value(target - total));
        }

        let solution = utxos.greedy(target);

        prop_assert!(
            !solution.is_empty(),
            "greedy() must always find a solution given enough available UTXOs"
        );

        prop_assert!(
            solution.iter().map(|u| u.value).sum::<u64>() >= target,
            "greedy() must reach the specified target amount"
        );
    }

    #[test]
    fn greedy_does_not_modify_input_when_fails(
        values in pvec(1u64..1_000_000_000, 1..10),
    ) {
        let dummy_utxos: Vec<Utxo> = values
            .into_iter()
            .map(dummy_utxo_from_value)
            .collect();

        let utxos = BitcoinUtxos::from(dummy_utxos);

        let total = utxos.total_value();

        let solution = utxos.greedy(total + 1);
        prop_assert!(solution.is_empty());
    }

    #[test]
    fn unsigned_tx_encoding_model(
        inputs in pvec(arb_unsigned_input(5_000u64..1_000_000_000), 1..20),
        outputs in pvec(arb_tx_out(), 1..20),
        lock_time in any::<u32>(),
    ) {
        let arb_tx = tx::UnsignedTransaction { inputs, outputs, lock_time };
        println!("{:?}", arb_tx);
        let btc_tx = unsigned_tx_to_bitcoin_tx(&arb_tx);
        println!("{:?}", serialize(&btc_tx));

        let tx_bytes = tx::encode_into(&arb_tx, Vec::<u8>::new());
        println!("{:?}", tx_bytes);
        let decoded_btc_tx: Transaction = deserialize(&tx_bytes).expect("failed to deserialize an unsigned transaction");

        prop_assert_eq!(serialize(&btc_tx), tx_bytes);
        prop_assert_eq!(&decoded_btc_tx, &btc_tx);
        prop_assert_eq!(&arb_tx.txid(), &btc_tx.txid().to_byte_array());
    }

    #[test]
    fn unsigned_tx_sighash_model(
        inputs_data in pvec(
            (
                arb_utxo(5_000u64..1_000_000_000),
                any::<u32>(),
                pvec(any::<u8>(), tx::PUBKEY_LEN)
            ),
            1..20
        ),
        outputs in pvec(arb_tx_out(), 1..20),
        lock_time in any::<u32>(),
    ) {
        let inputs: Vec<tx::UnsignedInput> = inputs_data
            .iter()
            .map(|(utxo, seq, _)| tx::UnsignedInput {
                previous_output: utxo.outpoint.clone(),
                value: utxo.value,
                sequence: *seq,
            })
            .collect();
        let arb_tx = tx::UnsignedTransaction { inputs, outputs, lock_time };
        let btc_tx = unsigned_tx_to_bitcoin_tx(&arb_tx);

        let sighasher = tx::TxSigHasher::new(&arb_tx);
        let mut btc_sighasher = SighashCache::new(btc_tx);

        for (i, (utxo, _, pubkey)) in inputs_data.iter().enumerate() {
            let mut buf = Vec::<u8>::new();
            let pkhash = tx::hash160(pubkey);

            sighasher.encode_sighash_data(&arb_tx.inputs[i], &pkhash, &mut buf);

            let mut btc_buf = Vec::<u8>::new();
            let script_code = p2wpkh_script_code(&pkhash);
            btc_sighasher.segwit_v0_encode_signing_data_to(&mut btc_buf, i, &script_code, Amount::from_sat(utxo.value), bitcoin::EcdsaSighashType::All)
                .expect("failed to encode sighash data");
            prop_assert_eq!(hex::encode(&buf), hex::encode(&btc_buf));

            let sighash = sighasher.sighash(&arb_tx.inputs[i], &pkhash);
            let btc_sighash = btc_sighasher.p2wsh_signature_hash(i, &script_code, Amount::from_sat(utxo.value), bitcoin::EcdsaSighashType::All).unwrap();
            prop_assert_eq!(hex::encode(sighash), hex::encode(btc_sighash));
        }
    }

    #[test]
    fn signed_tx_encoding_model(
        inputs in pvec(arb_signed_input(), 1..20),
        outputs in pvec(arb_tx_out(), 1..20),
        lock_time in any::<u32>(),
    ) {
        let arb_tx = tx::SignedTransaction { inputs, outputs, lock_time };
        let btc_tx = signed_tx_to_bitcoin_tx(&arb_tx);

        let tx_bytes = tx::encode_into(&arb_tx, Vec::<u8>::new());
        let decoded_btc_tx: Transaction = deserialize(&tx_bytes).expect("failed to deserialize an unsigned transaction");

        prop_assert_eq!(serialize(&btc_tx), tx_bytes);
        prop_assert_eq!(&decoded_btc_tx, &btc_tx);
        prop_assert_eq!(&arb_tx.wtxid(), &btc_tx.wtxid().to_byte_array());
        prop_assert_eq!(arb_tx.vsize(), btc_tx.vsize());
    }

    #[test]
    fn build_tx_splits_utxos(
        dummy_utxos in pvec(arb_utxo(5_000u64..1_000_000_000), 1..20),
        dst_pkhash in uniform20(any::<u8>()),
        main_pkhash in uniform20(any::<u8>()),
        fee_per_vbyte in 1000..2000u64,
    ) {
        prop_assume!(dst_pkhash != main_pkhash);

        let total_value = dummy_utxos.iter().map(|u| u.value).sum::<u64>();

        let target = total_value / 2;

        let utxos = BitcoinUtxos::from(dummy_utxos);

        let fee_estimate = utxos.estimate_fee(Some(target), fee_per_vbyte);

        let (unsigned_tx, fee) = utxos.build_unsigned_transaction(
            &BitcoinAddress::P2wpkhV0(main_pkhash),
            &BitcoinAddress::P2wpkhV0(dst_pkhash),
            target,
            fee_per_vbyte
        )
        .expect("failed to build transaction");

        let vsize = unsigned_tx.fake_sign().vsize() as u64;

        prop_assert_eq!(
            vsize,
            tx_vsize_estimate(unsigned_tx.inputs.len() as u64, unsigned_tx.outputs.len() as u64),
            "incorrect transaction vsize estimate"
        );

        let inputs_value = unsigned_tx.inputs.iter().map(|input| input.value).sum::<u64>();
        let outputs_value = unsigned_tx.outputs.iter().map(|output| output.value).sum::<u64>();

        let tx_fee = inputs_value - outputs_value;

        prop_assert!(tx_fee >= fee_estimate);
        prop_assert_eq!(fee, tx_fee);
        prop_assert!(inputs_value >= target);
        prop_assert!(tx_fee < target);
        prop_assert_eq!(tx_fee, fee_estimate, "incorrect transaction fee estimate");
    }

    #[test]
    fn check_output_order(
        dummy_utxos in pvec(arb_utxo(5_000u64..1_000_000_000), 1..20),
        dst_pkhash in uniform20(any::<u8>()),
        main_pkhash in uniform20(any::<u8>()),
        target in 50000..100000u64,
        fee_per_vbyte in 1000..2000u64,
    ) {
        prop_assume!(dst_pkhash != main_pkhash);
        let utxos = BitcoinUtxos::from(dummy_utxos);

        let (unsigned_tx, _) = utxos.build_unsigned_transaction(
            &BitcoinAddress::P2wpkhV0(main_pkhash),
            &BitcoinAddress::P2wpkhV0(dst_pkhash),
            target,
            fee_per_vbyte
        )
        .expect("failed to build transaction");

        prop_assert_eq!(&unsigned_tx.outputs.first().unwrap().address, &BitcoinAddress::P2wpkhV0(dst_pkhash));
        prop_assert_eq!(&unsigned_tx.outputs.last().unwrap().address, &BitcoinAddress::P2wpkhV0(main_pkhash));
    }

    #[test]
    fn build_tx_handles_change_from_inputs(
        dummy_utxos in pvec(arb_utxo(5_000u64..1_000_000_000), 10..20),
        dst_pkhash in uniform20(any::<u8>()),
        main_pkhash in uniform20(any::<u8>()),
        target in 50000..100000u64,
        fee_per_vbyte in 1000..2000u64,
    ) {
        prop_assume!(dst_pkhash != main_pkhash);
        let utxos = BitcoinUtxos::from(dummy_utxos);

        let value_by_outpoint: HashMap<_, _> = utxos
            .iter()
            .map(|utxo| (utxo.outpoint.clone(), utxo.value))
            .collect();

        let (unsigned_tx, fee) = utxos.build_unsigned_transaction(
            &BitcoinAddress::P2wpkhV0(main_pkhash),
            &BitcoinAddress::P2wpkhV0(dst_pkhash),
            target,
            fee_per_vbyte
        )
        .expect("failed to build transaction");

        let calc_fee = unsigned_tx.fake_sign().vsize() as u64 * fee_per_vbyte / 1000;

        prop_assert_eq!(fee, calc_fee);

        let inputs_value = unsigned_tx.inputs
            .iter()
            .map(|input| value_by_outpoint.get(&input.previous_output).unwrap())
            .sum::<u64>();

        prop_assert_eq!(
            &unsigned_tx.outputs,
            &vec![
                tx::TxOut {
                    value: target ,
                    address: BitcoinAddress::P2wpkhV0(dst_pkhash),
                },
                tx::TxOut {
                    value: inputs_value - target - fee,
                    address: BitcoinAddress::P2wpkhV0(main_pkhash),
                },
            ]
        );

        let change_output = unsigned_tx.outputs.last().unwrap().value;

        prop_assert_eq!(change_output, inputs_value - target - fee);
    }

    #[test]
    fn build_tx_does_not_modify_utxos_on_error(
        dummy_utxos in pvec(arb_utxo(5_000u64..1_000_000_000), 10..20),
        dst_pkhash in uniform20(any::<u8>()),
        main_pkhash in uniform20(any::<u8>()),
        fee_per_vbyte in 1000..2000u64,
    ) {
        let utxos = BitcoinUtxos::from(dummy_utxos);

        let utxos_copy = utxos.clone();

        let total_value = utxos.iter().map(|u| u.value).sum::<u64>();

        prop_assert_eq!(
            utxos.build_unsigned_transaction(
                &BitcoinAddress::P2wpkhV0(main_pkhash),
                &BitcoinAddress::P2wpkhV0(dst_pkhash),
                total_value * 2,
                fee_per_vbyte
            ).expect_err("build transaction should fail because the amount is too high"),
            BitcoinError::NotEnoughFunds
        );
        prop_assert_eq!(&utxos_copy, &utxos);

        prop_assert_eq!(
            utxos.build_unsigned_transaction(
                &BitcoinAddress::P2wpkhV0(main_pkhash),
                &BitcoinAddress::P2wpkhV0(dst_pkhash),
                1,
                fee_per_vbyte
            ).expect_err("build transaction should fail because the amount is too low to pay the fee"),
            BitcoinError::FeeTooHigh(utxos.estimate_fee(Some(1), fee_per_vbyte), 1)
        );
        prop_assert_eq!(&utxos_copy, &utxos);
    }

    #[test]
    fn btc_v0_p2wpkh_address_parsing(mut pkbytes in pvec(any::<u8>(), 32)) {
        use super::address::network_and_public_key_to_p2wpkh;
        pkbytes.insert(0, 0x02);

        for network in [BitcoinNetwork::Mainnet, BitcoinNetwork::Testnet, BitcoinNetwork::Regtest].iter() {
            let addr = network_and_public_key_to_p2wpkh(*network, &pkbytes);
            prop_assert_eq!(
                Ok(BitcoinAddress::P2wpkhV0(tx::hash160(&pkbytes))),
                BitcoinAddress::parse(&addr, *network)
            );
        }
    }

    #[test]
    fn btc_address_parsing_model(mut pkbytes in pvec(any::<u8>(), 32)) {
        pkbytes.insert(0, 0x02);

        let pk_result = bitcoin::PublicKey::from_slice(&pkbytes);

        prop_assume!(pk_result.is_ok());

        let pk = pk_result.unwrap();
        let pkhash = tx::hash160(&pkbytes);

        for network in [BitcoinNetwork::Mainnet, BitcoinNetwork::Testnet, BitcoinNetwork::Regtest].iter() {
            let btc_net = btc_network_to_network(*network);
            let btc_addr = bitcoin::Address::p2pkh(&pk, btc_net);
            prop_assert_eq!(
                Ok(BitcoinAddress::P2pkh(tx::hash160(&pkbytes))),
                BitcoinAddress::parse(&btc_addr.to_string(), *network)
            );

            let btc_addr = bitcoin::Address::p2wpkh(&pk, btc_net).unwrap();
            prop_assert_eq!(
                Ok(BitcoinAddress::P2wpkhV0(pkhash)),
                BitcoinAddress::parse(&btc_addr.to_string(), *network)
            );
        }
    }

    #[test]
    fn btc_address_display_model(address in arb_address()) {
        for network in [BitcoinNetwork::Mainnet, BitcoinNetwork::Testnet].iter() {
            let addr_str = address.display(*network);
            let btc_addr = address_to_btc_address(&address, btc_network_to_network(*network));
            prop_assert_eq!(btc_addr, bitcoin::Address::from_str(&addr_str).unwrap());
        }
    }

    #[test]
    fn address_roundtrip(address in arb_address()) {
        for network in [BitcoinNetwork::Mainnet, BitcoinNetwork::Testnet, BitcoinNetwork::Regtest].iter() {
            let addr_str = address.display(*network);
            prop_assert_eq!(BitcoinAddress::parse(&addr_str, *network), Ok(address.clone()));
        }
    }

    #[test]
    fn sec1_to_der_positive_parses(sig in pvec(1u8..0x0f, 64)) {
        use simple_asn1::{from_der, ASN1Block::{Sequence, Integer}};

        let der = super::signature::sec1_to_der(&sig);
        let decoded = from_der(&der).expect("failed to decode DER");
        if let[Sequence(_, items)] = &decoded[..] {
            if let [Integer(_, r), Integer(_, s)] = &items[..] {
                let (_, r_be) = r.to_bytes_be();
                let (_, s_be) = s.to_bytes_be();
                prop_assert_eq!(&r_be[..], &sig[..32]);
                prop_assert_eq!(&s_be[..], &sig[32..]);
                return Ok(());
            }
        }
        prop_assert!(false, "expected a DER sequence with two items, got: {:?}", decoded);
    }

    #[test]
    fn sec1_to_der_non_zero_parses(sig in pvec(any::<u8>(), 64)) {
        use simple_asn1::{from_der, ASN1Block::{Sequence, Integer}};

        prop_assume!(sig[..32].iter().any(|x| *x > 0));
        prop_assume!(sig[32..].iter().any(|x| *x > 0));

        let der = super::signature::sec1_to_der(&sig);
        let decoded = from_der(&der).expect("failed to decode DER");

        if let[Sequence(_, items)] = &decoded[..] {
            if let [Integer(_, _r), Integer(_, _s)] = &items[..] {
                return Ok(());
            }
        }
        prop_assert!(false, "expected a DER sequence with two items, got: {:?}", decoded);
    }

    #[test]
    fn encode_valid_signatures(sig in pvec(any::<u8>(), 64)) {
        prop_assume!(sig[..32].iter().any(|x| *x > 0));
        prop_assume!(sig[32..].iter().any(|x| *x > 0));

        let encoded = super::signature::EncodedSignature::from_sec1(&sig);
        super::signature::validate_encoded_signature(encoded.as_slice()).expect("invalid signature");
    }


    #[test]
    fn test_fee_range(
        dummy_utxos in pvec(arb_utxo(5_000u64..1_000_000_000), 10..20),
        amount in option::of(any::<u64>()),
        fee_per_vbyte in 2000..10000u64,
    ) {
        const SMALLEST_TX_SIZE_VBYTES: u64 = 140; // one input, two outputs

        let utxos = BitcoinUtxos::from(dummy_utxos);

        let estimate = utxos.estimate_fee(amount, fee_per_vbyte);
        let lower_bound = SMALLEST_TX_SIZE_VBYTES * fee_per_vbyte / 1000;

        prop_assert!(
            estimate >= lower_bound,
            "The fee estimate {} is below the lower bound {}",
            estimate,
            lower_bound
        );
    }
}

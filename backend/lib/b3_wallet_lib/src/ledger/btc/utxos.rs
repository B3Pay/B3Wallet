use super::{error::BitcoinError, utils::mock_signer};
use crate::ledger::types::{BtcOutPoint, BtcTransaction, BtcTxOut};
use bitcoin::{absolute::LockTime, hashes::Hash, Address, Script, Transaction, TxIn, Txid};
use ic_cdk::api::management_canister::bitcoin::{GetUtxosResponse, Utxo};

pub struct BtcUtxos(Vec<Utxo>);

impl TryFrom<GetUtxosResponse> for BtcUtxos {
    type Error = BitcoinError;

    fn try_from(utxos: GetUtxosResponse) -> Result<Self, Self::Error> {
        if utxos.utxos.is_empty() {
            return Err(BitcoinError::NoUtxos);
        }

        Ok(Self(utxos.utxos))
    }
}

impl TryFrom<Vec<Utxo>> for BtcUtxos {
    type Error = BitcoinError;

    fn try_from(utxos: Vec<Utxo>) -> Result<Self, Self::Error> {
        if utxos.is_empty() {
            return Err(BitcoinError::NoUtxos);
        }

        Ok(Self(utxos))
    }
}

impl BtcUtxos {
    pub fn build_transaction(
        &self,
        own_address: &Address,
        recipient: &Address,
        amount: u64,
        fee_rate: u64,
    ) -> Result<Transaction, BitcoinError> {
        let mut total_fee = 0;
        loop {
            let mut transaction =
                self.build_transaction_with_fee(&own_address, recipient, amount, total_fee)?;

            for (_, input) in transaction.input.iter_mut().enumerate() {
                input.script_sig = mock_signer();
                input.witness.clear();
            }

            let transaction_size = transaction.vsize() as u64;
            let fee = (fee_rate * transaction_size) / 1000;

            // If the fee is correct, we're done.
            if total_fee == fee {
                transaction.input.iter_mut().for_each(|input| {
                    input.script_sig = Script::empty().into();
                    input.witness.clear();
                });

                return Ok(transaction);
            }

            total_fee = fee;

            // If the fee is too high, we're done.
            if fee > amount {
                return Err(BitcoinError::FeeTooHigh(fee, amount));
            }
        }
    }

    pub fn build_transaction_with_fee(
        &self,
        own_address: &Address,
        dst_address: &Address,
        amount: u64,
        fee: u64,
    ) -> Result<Transaction, BitcoinError> {
        // Assume that any amount below this threshold is dust.
        const DUST_THRESHOLD: u64 = 1_000;

        let mut transaction = BtcTransaction {
            version: 2,
            input: Vec::new(),
            output: Vec::new(),
            lock_time: LockTime::ZERO,
        };

        let mut total_spent = 0;
        for utxo in self.0.iter().rev() {
            total_spent += utxo.value;

            let mut tx_in = TxIn::default();

            let txid = Txid::from_slice(&utxo.outpoint.txid).unwrap();
            tx_in.previous_output = BtcOutPoint::new(txid, utxo.outpoint.vout);

            transaction.input.push(tx_in);

            if total_spent >= amount + fee {
                // We have enough inputs to cover the amount we want to spend.
                break;
            }
        }

        if total_spent < amount + fee {
            return Err(BitcoinError::InsufficientBalance(total_spent, amount + fee));
        }

        transaction.output.push(BtcTxOut {
            script_pubkey: dst_address.script_pubkey(),
            value: amount,
        });

        let remaining_amount = total_spent - amount - fee;

        if remaining_amount >= DUST_THRESHOLD {
            transaction.output.push(BtcTxOut {
                script_pubkey: own_address.script_pubkey(),
                value: remaining_amount,
            });
        }

        Ok(transaction)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::ledger::{
        btc::{network::BtcNetwork, utxos::BtcUtxos},
        chain::{Chain, ChainTrait},
        ecdsa::EcdsaPublicKey,
        ledger::Ledger,
        types::{ChainEnum, ChainMap},
    };

    use super::*;
    use b3_utils::{ledger::AccountIdentifier, mocks::id_mock, Subaccount};
    use ic_cdk::api::management_canister::bitcoin::{Outpoint, Utxo};

    #[test]
    fn test_build_unsigned_transaction() {
        let subaccount = Subaccount([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let account_identifier = AccountIdentifier::new(id_mock(), subaccount.clone());

        let icp_chain = Chain::new_icp_chain(subaccount.clone());

        assert_eq!(icp_chain.address(), account_identifier.to_string());

        let mut chains = ChainMap::new();

        chains.insert(ChainEnum::ICP, icp_chain);

        let mut ledger = Ledger {
            public_key: None,
            subaccount: subaccount.clone(),
            chains,
        };

        let ecdsa = vec![
            3, 94, 114, 171, 76, 217, 209, 126, 120, 169, 209, 205, 226, 55, 21, 238, 204, 199,
            153, 192, 65, 30, 59, 177, 153, 39, 80, 76, 185, 200, 51, 255, 218,
        ];

        ledger.set_ecdsa_public_key(ecdsa.clone()).unwrap();

        let utxos = BtcUtxos::try_from(vec![
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        139, 171, 81, 80, 180, 153, 27, 232, 110, 73, 221, 62, 162, 144, 67, 185,
                        61, 207, 175, 9, 26, 144, 153, 242, 243, 148, 56, 186, 112, 246, 164, 230,
                    ],
                    vout: 0,
                },
                value: 10_000_000,
                height: 0,
            },
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        139, 171, 81, 80, 180, 153, 27, 232, 110, 73, 221, 62, 162, 144, 67, 185,
                        61, 207, 175, 9, 26, 144, 153, 242, 243, 148, 56, 186, 112, 246, 164, 230,
                    ],
                    vout: 0,
                },
                value: 10_000_000,
                height: 0,
            },
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        139, 171, 81, 80, 180, 153, 27, 232, 110, 73, 221, 62, 162, 144, 67, 185,
                        61, 207, 175, 9, 26, 144, 153, 242, 243, 148, 56, 186, 112, 246, 164, 230,
                    ],
                    vout: 0,
                },
                value: 10_000_000,
                height: 0,
            },
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        139, 171, 81, 80, 180, 153, 27, 232, 110, 73, 221, 62, 162, 144, 67, 185,
                        61, 207, 175, 9, 26, 144, 153, 242, 243, 148, 56, 186, 112, 246, 164, 230,
                    ],
                    vout: 0,
                },
                value: 10_000_000,
                height: 0,
            },
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        139, 171, 81, 80, 180, 153, 27, 232, 110, 73, 221, 62, 162, 144, 67, 185,
                        61, 207, 175, 9, 26, 144, 153, 242, 243, 148, 56, 186, 112, 246, 164, 230,
                    ],
                    vout: 0,
                },
                value: 10_000_000,
                height: 0,
            },
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        139, 171, 81, 80, 180, 153, 27, 232, 110, 73, 221, 62, 162, 144, 67, 185,
                        61, 207, 175, 9, 26, 144, 153, 242, 243, 148, 56, 186, 112, 246, 164, 230,
                    ],
                    vout: 0,
                },
                value: 10_000_000,
                height: 0,
            },
            Utxo {
                outpoint: Outpoint {
                    txid: vec![
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4,
                        5, 6, 7, 8, 9, 8, 9,
                    ],
                    vout: 1,
                },
                value: 50_000_000,
                height: 1,
            },
        ])
        .unwrap();

        let recipient = Address::from_str("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq")
            .unwrap()
            .assume_checked();

        let chain = Chain::new_btc_chain(
            BtcNetwork::Regtest,
            subaccount,
            EcdsaPublicKey(ecdsa.clone()),
        )
        .unwrap();

        ledger.insert_chain(ChainEnum::BTC(BtcNetwork::Regtest), chain.clone());

        let btc_chain = ledger.chain(&ChainEnum::BTC(BtcNetwork::Regtest)).unwrap();

        let own_address = chain.btc().unwrap().btc_address().unwrap();

        assert_eq!(btc_chain.address(), own_address.to_string());

        let public_key = chain.btc().unwrap().btc_public_key().unwrap();

        let tx = utxos
            .build_transaction(&own_address, &recipient, 100_000_000, 2000)
            .unwrap();

        assert_eq!(public_key, EcdsaPublicKey(ecdsa).btc_public_key().unwrap());

        assert_eq!(tx.input.len(), 7);

        assert_eq!(tx.output.len(), 2);

        assert_eq!(tx.output[0].value, 100_000_000);

        assert_eq!(tx.output[0].script_pubkey, recipient.script_pubkey());

        assert_eq!(tx.input[0].previous_output.vout, 1);

        assert_eq!(tx.input[0].witness.len(), 0);

        assert_eq!(tx.input[0].script_sig.len(), 0);
    }
}

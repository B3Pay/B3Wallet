use std::collections::BTreeSet;

use super::{
    address::BitcoinAddress,
    error::BitcoinError,
    tx::{TxOut, UnsignedTransaction},
    types::{OutPoint, Utxo},
};
use crate::ledger::btc::{tx::UnsignedInput, utils::tx_vsize_estimate};
use ic_cdk::api::management_canister::bitcoin::GetUtxosResponse;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitcoinUtxos(BTreeSet<Utxo>);

impl BitcoinUtxos {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn from(utxos: Vec<Utxo>) -> Self {
        Self(utxos.into_iter().collect())
    }

    pub fn insert(&mut self, utxo: Utxo) -> bool {
        self.0.insert(utxo)
    }

    pub fn remove(&mut self, utxo: &Utxo) -> bool {
        self.0.remove(utxo)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Utxo> {
        self.0.iter()
    }

    pub fn contains(&self, utxo: &Utxo) -> bool {
        self.0.contains(utxo)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the total value of the UTXOs.
    /// The value is in satoshi.
    pub fn total_value(&self) -> u64 {
        self.0.iter().map(|u| u.value).sum()
    }

    /// Returns the list of UTXOs.
    /// The list is sorted by value, descending.
    pub fn utxos(&self) -> Vec<Utxo> {
        self.0.iter().cloned().collect()
    }

    /// Computes an estimate for the fee of a transaction.
    ///
    /// Arguments:
    /// * `amount` - The amount to send, in satoshi.
    /// * `fee_per_vbyte` - The fee per vbyte, in satoshi.
    ///
    /// Returns:
    /// * The estimated fee, in satoshi.
    pub fn estimate_fee(&self, amount: u64, fee_per_vbyte: u64) -> u64 {
        // One output for the caller and one for the change.
        const DEFAULT_OUTPUT_COUNT: u64 = 2;
        let mut selected_utxos = Vec::new();
        let mut total_value = 0u64;
        let mut total_fee = 0u64;

        for utxo in self.0.iter().rev() {
            selected_utxos.push(utxo.clone());
            total_value += utxo.value;

            let estimated_vsize =
                tx_vsize_estimate(selected_utxos.len() as u64, DEFAULT_OUTPUT_COUNT);

            total_fee = estimated_vsize * fee_per_vbyte / 1000;

            if total_value >= amount + total_fee {
                break;
            }
        }

        total_fee
    }

    pub fn build_unsigned_transaction(
        &self,
        own_address: &BitcoinAddress,
        dst_address: &BitcoinAddress,
        amount: u64,
        fee_per_vbyte: u64,
    ) -> Result<(UnsignedTransaction, u64), BitcoinError> {
        assert!(!self.is_empty());

        const DUST_THRESHOLD: u64 = 1_000;
        const SEQUENCE_RBF_ENABLED: u32 = 0xfffffffd;

        let mut selected_utxos = Vec::new();
        let mut total_value = 0u64;
        let mut total_fee = 0u64;

        for utxo in self.0.iter().rev() {
            selected_utxos.push(utxo.clone());
            total_value += utxo.value;

            let estimated_vsize = tx_vsize_estimate(selected_utxos.len() as u64, 2);

            total_fee = estimated_vsize * fee_per_vbyte / 1000;

            if total_fee > amount {
                return Err(BitcoinError::FeeTooHigh(total_fee, amount));
            }

            if total_value >= amount + total_fee {
                break;
            }
        }

        if total_value < amount + total_fee {
            return Err(BitcoinError::InsufficientBalance(
                total_value,
                amount + total_fee,
            ));
        }

        let mut unsigned_transaction = UnsignedTransaction {
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        };

        for utxo in selected_utxos {
            let tx_in = UnsignedInput {
                previous_output: OutPoint::new(utxo.outpoint.txid.clone(), utxo.outpoint.vout),
                sequence: SEQUENCE_RBF_ENABLED,
                value: utxo.value,
            };

            unsigned_transaction.inputs.push(tx_in);
        }

        unsigned_transaction.outputs.push(TxOut {
            address: dst_address.clone(),
            value: amount,
        });

        let remaining_amount = total_value - amount - total_fee;
        if remaining_amount >= DUST_THRESHOLD {
            unsigned_transaction.outputs.push(TxOut {
                address: own_address.clone(),
                value: remaining_amount,
            });
        }

        Ok((unsigned_transaction, total_fee))
    }

    pub fn build_unsigned_transaction_with_fee(
        &self,
        own_address: &BitcoinAddress,
        dst_address: &BitcoinAddress,
        amount: u64,
        fee: u64,
    ) -> Result<UnsignedTransaction, BitcoinError> {
        const DUST_THRESHOLD: u64 = 1_000;
        const SEQUENCE_RBF_ENABLED: u32 = 0xfffffffd;

        let mut unsigned_transaction = UnsignedTransaction {
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        };

        let mut total_spent = 0;
        for utxo in self.0.iter() {
            if total_spent >= amount + fee {
                break;
            }
            total_spent += utxo.value;

            let tx_in = UnsignedInput {
                previous_output: OutPoint::new(utxo.outpoint.txid.clone(), utxo.outpoint.vout),
                sequence: SEQUENCE_RBF_ENABLED,
                value: utxo.value,
            };

            unsigned_transaction.inputs.push(tx_in);
        }

        if total_spent < amount + fee {
            return Err(BitcoinError::NotEnoughFunds);
        }

        unsigned_transaction.outputs.push(TxOut {
            address: dst_address.clone(),
            value: amount,
        });

        let remaining_amount = total_spent - amount - fee;
        if remaining_amount >= DUST_THRESHOLD {
            unsigned_transaction.outputs.push(TxOut {
                address: own_address.clone(),
                value: remaining_amount,
            });
        }

        Ok(unsigned_transaction)
    }
}

impl TryFrom<GetUtxosResponse> for BitcoinUtxos {
    type Error = BitcoinError;

    fn try_from(utxos: GetUtxosResponse) -> Result<Self, Self::Error> {
        if utxos.utxos.is_empty() {
            return Err(BitcoinError::NoUtxos);
        }

        let mut set = BTreeSet::new();

        for utxo in utxos.utxos {
            set.insert(Utxo {
                outpoint: OutPoint {
                    txid: utxo.outpoint.txid,
                    vout: utxo.outpoint.vout,
                },
                value: utxo.value,
                height: utxo.height,
            });
        }

        Ok(Self(set))
    }
}

impl TryFrom<Vec<Utxo>> for BitcoinUtxos {
    type Error = BitcoinError;

    fn try_from(utxos: Vec<Utxo>) -> Result<Self, Self::Error> {
        if utxos.is_empty() {
            return Err(BitcoinError::NoUtxos);
        }

        let mut set = BTreeSet::new();

        for utxo in utxos {
            set.insert(utxo);
        }

        Ok(Self(set))
    }
}

#[cfg(test)]
mod test {

    use crate::ledger::{
        btc::types::OutPoint,
        btc::{network::BitcoinNetwork, utxos::BitcoinUtxos},
        chain::{Chain, ChainTrait},
        ecdsa::ECDSAPublicKey,
        ledger::Ledger,
        types::{ChainEnum, ChainMap},
    };

    use super::*;
    use b3_utils::{ledger::AccountIdentifier, mocks::id_mock, Subaccount};

    #[test]
    fn test_build_unsigned_transaction() {
        let subaccount = Subaccount([
            8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);

        let account_identifier = AccountIdentifier::new(id_mock(), Some(subaccount.clone()));

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

        let utxos = BitcoinUtxos::from(vec![
            Utxo {
                outpoint: OutPoint {
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
                outpoint: OutPoint {
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
                outpoint: OutPoint {
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
                outpoint: OutPoint {
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
                outpoint: OutPoint {
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
                outpoint: OutPoint {
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
                outpoint: OutPoint {
                    txid: vec![
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4,
                        5, 6, 7, 8, 9, 8, 9,
                    ],
                    vout: 1,
                },
                value: 50_000_000,
                height: 1,
            },
        ]);

        let recipient = BitcoinAddress::parse(
            "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
            BitcoinNetwork::Mainnet,
        )
        .unwrap();

        let chain = Chain::new_btc_chain(
            BitcoinNetwork::Regtest,
            subaccount,
            ECDSAPublicKey::new(ecdsa.clone()),
        )
        .unwrap();

        ledger.insert_chain(ChainEnum::BTC(BitcoinNetwork::Regtest), chain.clone());

        let btc_chain = ledger
            .chain(&ChainEnum::BTC(BitcoinNetwork::Regtest))
            .unwrap();

        let own_address = chain.btc().unwrap().btc_address();

        assert_eq!(
            btc_chain.address(),
            own_address.display(BitcoinNetwork::Mainnet)
        );

        let public_key = chain.btc().unwrap().btc_public_key().unwrap();

        let (tx, fee) = utxos
            .build_unsigned_transaction(&own_address, &recipient, 100_000_000, 2000)
            .unwrap();

        assert!(fee > 0 && fee < 100_000_000);

        assert_eq!(
            public_key,
            ECDSAPublicKey::new(ecdsa).btc_public_key().unwrap()
        );

        assert_eq!(tx.inputs.len(), 7);

        assert_eq!(tx.outputs.len(), 2);

        assert_eq!(tx.outputs[0].value, 100_000_000u64);

        assert_eq!(tx.outputs[0].address, recipient);

        assert_eq!(tx.inputs[0].previous_output.vout, 1);

        assert_eq!(tx.inputs[0].value, 50_000_000u64);

        assert_eq!(tx.inputs[0].sequence, 0xfffffffd);
    }
}

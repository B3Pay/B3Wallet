use std::collections::BTreeSet;

use super::{
    address::BitcoinAddress,
    error::BitcoinError,
    tx::{TxOut, UnsignedTransaction},
    types::{OutPoint, Satoshi, Utxo},
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

    /// Computes an estimate for the retrieve_btc fee.
    ///
    /// Arguments:
    ///   * `available_utxos` - the list of UTXOs available to the minter.
    ///   * `maybe_amount` - the withdrawal amount.
    ///   * `median_fee_millisatoshi_per_vbyte` - the median network fee, in millisatoshi per vbyte.
    pub fn estimate_fee(
        &self,
        maybe_amount: Option<u64>,
        median_fee_millisatoshi_per_vbyte: u64,
    ) -> u64 {
        const DEFAULT_INPUT_COUNT: u64 = 3;
        // One output for the caller and one for the change.
        const DEFAULT_OUTPUT_COUNT: u64 = 2;
        let input_count = match maybe_amount {
            Some(amount) => {
                // We simulate the algorithm that selects UTXOs for the
                // specified amount. If the withdrawal rate is low, we
                // should get the exact number of inputs that the minter
                // will use.
                let selected_utxos = self.greedy(amount);

                if !selected_utxos.is_empty() {
                    selected_utxos.len() as u64
                } else {
                    DEFAULT_INPUT_COUNT
                }
            }
            None => DEFAULT_INPUT_COUNT,
        };

        let vsize = tx_vsize_estimate(input_count, DEFAULT_OUTPUT_COUNT);

        // We subtract one from the outputs because the minter's output
        // does not participate in fees distribution.
        let bitcoin_fee =
            vsize * median_fee_millisatoshi_per_vbyte / 1000 / (DEFAULT_OUTPUT_COUNT - 1).max(1);

        bitcoin_fee
    }
    /// Selects a subset of UTXOs with the specified total target value.
    ///
    /// If there are no UTXOs matching the criteria, returns an empty vector.
    ///
    /// PROPERTY: sum(u.value for u in available_set) ≥ target ⇒ !solution.is_empty()
    /// POSTCONDITION: !solution.is_empty() ⇒ sum(u.value for u in solution) ≥ target
    /// POSTCONDITION:  solution.is_empty() ⇒ available_utxos did not change.
    pub fn greedy(&self, target: u64) -> Vec<Utxo> {
        let mut utxo_clone = self.0.clone();

        let mut solution = vec![];
        let mut goal = target;

        while goal > 0 {
            let utxo = match utxo_clone.iter().max_by_key(|u| u.value) {
                Some(max_utxo) if max_utxo.value < goal => max_utxo.clone(),
                Some(_) => self
                    .iter()
                    .filter(|u| u.value >= goal)
                    .min_by_key(|u| u.value)
                    .cloned()
                    .expect("bug: there must be at least one UTXO matching the criteria"),
                None => {
                    // Not enough available UTXOs to satisfy the request.
                    for u in solution {
                        utxo_clone.insert(u);
                    }
                    return vec![];
                }
            };
            goal = goal.saturating_sub(utxo.value);
            assert!(utxo_clone.remove(&utxo));
            solution.push(utxo);
        }

        debug_assert!(
            solution.is_empty() || solution.iter().map(|u| u.value).sum::<u64>() >= target
        );

        solution
    }

    /// Builds a transaction that moves BTC to the specified destination accounts
    /// using the UTXOs that the minter owns. The receivers pay the fee.
    ///
    /// Sends the change back to the specified minter main address.
    ///
    /// # Arguments
    ///
    /// * `minter_utxos` - The set of all UTXOs minter owns
    /// * `outputs` - The destination BTC addresses and respective amounts.
    /// * `main_address` - The BTC address of the minter's main account do absorb the change.
    /// * `fee_per_vbyte` - The current 50th percentile of BTC fees, in millisatoshi/byte
    ///
    /// # Panics
    ///
    /// This function panics if the `outputs` vector is empty as it indicates a bug
    /// in the caller's code.
    ///
    /// # Success case properties
    ///
    /// * The total value of minter UTXOs decreases at least by the amount.
    /// ```text
    /// sum([u.value | u ∈ minter_utxos']) ≤ sum([u.value | u ∈ minter_utxos]) - amount
    /// ```
    ///
    /// * If the transaction inputs exceed the amount, the minter gets the change.
    /// ```text
    /// inputs_value(tx) > amount ⇒ out_value(tx, main_pubkey) >= inputs_value(tx) - amount
    /// ```
    ///
    /// * If the transaction inputs are equal to the amount, all tokens go to the receiver.
    /// ```text
    /// sum([value(in) | in ∈ tx.inputs]) = amount ⇒ tx.outputs == { value = amount - fee(tx); pubkey = dst_pubkey }
    /// ```
    ///
    ///  * The last output of the transaction is the minter's fee + the minter's change.
    /// ```text
    /// value(last_out) == minter_fee + minter_change
    /// ```
    ///
    /// # Error case properties
    ///
    /// * In case of errors, the function does not modify the inputs.
    /// ```text
    /// result.is_err() => minter_utxos' == minter_utxos
    /// ```
    ///
    pub fn build_unsigned_transaction(
        &self,
        own_address: &BitcoinAddress,
        dst_address: &BitcoinAddress,
        amount: Satoshi,
        fee_per_vbyte: u64,
    ) -> Result<(UnsignedTransaction, u64), BitcoinError> {
        assert!(!self.is_empty());

        /// Having a sequence number lower than (0xffffffff - 1) signals the use of replacement by fee.
        /// It allows us to increase the fee of a transaction already sent to the mempool.
        /// The rbf option is used in `resubmit_retrieve_btc`.
        /// https://github.com/bitcoin/bips/blob/master/bip-0125.mediawiki
        const SEQUENCE_RBF_ENABLED: u32 = 0xfffffffd;

        let input_utxos = self.greedy(amount);

        if input_utxos.is_empty() {
            return Err(BitcoinError::NotEnoughFunds);
        }

        let inputs_value = input_utxos.iter().map(|u| u.value).sum::<u64>();

        debug_assert!(inputs_value >= amount);

        let tx_outputs: Vec<TxOut> = vec![
            // The first output is the receiver's address.
            TxOut {
                address: dst_address.clone(),
                value: amount,
            },
            // The second output is the change.
            // The change is sent back to the owner address.
            TxOut {
                address: own_address.clone(),
                value: inputs_value - amount,
            },
        ];

        let total_output_value = tx_outputs.iter().map(|out| out.value).sum::<u64>();

        debug_assert_eq!(total_output_value, inputs_value);

        let mut unsigned_tx = UnsignedTransaction {
            inputs: input_utxos
                .iter()
                .map(|utxo| UnsignedInput {
                    previous_output: utxo.outpoint.clone(),
                    value: utxo.value,
                    sequence: SEQUENCE_RBF_ENABLED,
                })
                .collect(),
            outputs: tx_outputs,
            lock_time: 0,
        };

        let tx_vsize = unsigned_tx.fake_sign().vsize();
        let fee = (tx_vsize as u64 * fee_per_vbyte) / 1000;

        if fee > amount {
            return Err(BitcoinError::FeeTooHigh(fee, amount));
        }

        // The default dustRelayFee is 3 sat/vB,
        // which translates to a dust threshold of 546 satoshi for P2PKH outputs.
        // The threshold for other types is lower,
        // so we simply use 546 satoshi as the minimum amount per output.
        const MIN_OUTPUT_AMOUNT: u64 = 546;

        unsigned_tx.outputs[1].value = unsigned_tx.outputs[1]
            .value
            .saturating_sub(fee)
            .max(MIN_OUTPUT_AMOUNT);

        debug_assert_eq!(
            inputs_value,
            fee + unsigned_tx.outputs.iter().map(|u| u.value).sum::<u64>()
        );

        Ok((unsigned_tx, fee))
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

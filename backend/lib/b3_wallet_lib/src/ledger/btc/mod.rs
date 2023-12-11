pub mod address;
pub mod api;
pub mod btc;
pub mod error;
pub mod network;
pub mod signature;
pub mod test;
pub mod tx;
pub mod types;
pub mod utils;
pub mod utxos;

use address::BitcoinAddress;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::bitcoin::MillisatoshiPerByte;
use memo::Status;
use num_traits::ToPrimitive;
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use types::{OutPoint, Satoshi, Utxo};

use self::network::BtcNetwork;

/// Time constants
const SEC_NANOS: u64 = 1_000_000_000;
const MIN_NANOS: u64 = 60 * SEC_NANOS;
/// The minimum number of pending request in the queue before we try to make
/// a batch transaction.
pub const MIN_PENDING_REQUESTS: usize = 20;
pub const MAX_REQUESTS_PER_BATCH: usize = 100;

/// The constants used to compute the minter's fee to cover its own cycle consumption.
/// The values are set to cover the cycle cost on a 28-node subnet.
pub const MINTER_FEE_PER_INPUT: u64 = 246;
pub const MINTER_FEE_PER_OUTPUT: u64 = 7;
pub const MINTER_FEE_CONSTANT: u64 = 52;

/// The minimum fee increment for transaction resubmission.
/// See https://en.bitcoin.it/wiki/Miner_fees#Relaying for more detail.
pub const MIN_RELAY_FEE_PER_VBYTE: MillisatoshiPerByte = 1_000;

/// The minimum time the minter should wait before replacing a stuck transaction.
pub const MIN_RESUBMISSION_DELAY: Duration = Duration::from_secs(24 * 60 * 60);

/// The maximum memo size of a transaction on the ckBTC ledger.
/// The ckBTC minter requires at least 69 bytes, we choose 80
/// to have some room for future modifications.
pub const CKBTC_LEDGER_MEMO_SIZE: u16 = 80;

#[derive(Clone, serde::Serialize, Deserialize, Debug)]
pub enum Priority {
    P0,
    P1,
}

#[derive(Clone, serde::Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub timestamp: u64,
    pub priority: Priority,
    pub file: String,
    pub line: u32,
    pub message: String,
    pub counter: u64,
}

#[derive(Clone, Default, serde::Serialize, Deserialize, Debug)]
pub struct Log {
    pub entries: Vec<LogEntry>,
}

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub struct MinterInfo {
    pub min_confirmations: u32,
    pub retrieve_btc_min_amount: u64,
    pub kyt_fee: u64,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ECDSAPublicKey {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

struct SignTxRequest {
    key_name: String,
    network: BtcNetwork,
    ecdsa_public_key: ECDSAPublicKey,
    unsigned_tx: tx::UnsignedTransaction,
    change_output: state::ChangeOutput,
    outpoint_account: BTreeMap<OutPoint, Account>,
    /// The original requests that we keep around to place back to the queue
    /// if the signature fails.
    requests: Vec<state::RetrieveBtcRequest>,
    /// The list of UTXOs we use as transaction inputs.
    utxos: Vec<Utxo>,
}

/// Updates the UTXOs for the main account of the minter to pick up change from
/// previous retrieve BTC requests.
async fn fetch_main_utxos(main_account: &Account, main_address: &BitcoinAddress) -> Vec<Utxo> {
    let (btc_network, min_confirmations) =
        state::read_state(|s| (s.btc_network, s.min_confirmations));

    let utxos = match management::get_utxos(
        btc_network,
        &main_address.display(btc_network),
        min_confirmations,
        management::CallSource::Minter,
    )
    .await
    {
        Ok(response) => response.utxos,
        Err(e) => {
            log!(
                P0,
                "[fetch_main_utxos]: failed to fetch UTXOs for the main address {}: {}",
                main_address.display(btc_network),
                e
            );
            return vec![];
        }
    };

    state::read_state(|s| match s.utxos_state_addresses.get(main_account) {
        Some(known_utxos) => utxos
            .into_iter()
            .filter(|u| !known_utxos.contains(u))
            .collect(),
        None => utxos,
    })
}

/// Returns the minimum withdrawal amount based on the current median fee rate (in millisatoshi per byte).
/// The returned amount is in satoshi.
fn compute_min_withdrawal_amount(median_fee_rate_e3s: MillisatoshiPerByte) -> u64 {
    const PER_REQUEST_RBF_BOUND: u64 = 22_100;
    const PER_REQUEST_VSIZE_BOUND: u64 = 221;
    const PER_REQUEST_MINTER_FEE_BOUND: u64 = 305;
    const PER_REQUEST_KYT_FEE: u64 = 2_000;

    let median_fee_rate = median_fee_rate_e3s / 1_000;
    ((PER_REQUEST_RBF_BOUND
        + PER_REQUEST_VSIZE_BOUND * median_fee_rate
        + PER_REQUEST_MINTER_FEE_BOUND
        + PER_REQUEST_KYT_FEE)
        / 50_000)
        * 50_000
        + 100_000
}

/// Returns an estimate for transaction fees in millisatoshi per vbyte. Returns
/// None if the bitcoin canister is unavailable or does not have enough data for
/// an estimate yet.
pub async fn estimate_fee_per_vbyte() -> Option<MillisatoshiPerByte> {
    /// The default fee we use on regtest networks if there are not enough data
    /// to compute the median fee.
    const DEFAULT_FEE: MillisatoshiPerByte = 5_000;

    let btc_network = state::read_state(|s| s.btc_network);
    match management::get_current_fees(btc_network).await {
        Ok(fees) => {
            if btc_network == BtcNetwork::Regtest {
                return Some(DEFAULT_FEE);
            }
            if fees.len() >= 100 {
                state::mutate_state(|s| {
                    s.last_fee_per_vbyte = fees.clone();
                    s.retrieve_btc_min_amount = compute_min_withdrawal_amount(fees[50]);
                });
                Some(fees[50])
            } else {
                log!(
                    P0,
                    "[estimate_fee_per_vbyte]: not enough data points ({}) to compute the fee",
                    fees.len()
                );
                None
            }
        }
        Err(err) => {
            log!(
                P0,
                "[estimate_fee_per_vbyte]: failed to get median fee per vbyte: {}",
                err
            );
            None
        }
    }
}

fn finalization_time_estimate(min_confirmations: u32, network: BtcNetwork) -> Duration {
    Duration::from_nanos(
        min_confirmations as u64
            * match network {
                BtcNetwork::Mainnet => 10 * MIN_NANOS,
                BtcNetwork::Testnet => MIN_NANOS,
                BtcNetwork::Regtest => SEC_NANOS,
            },
    )
}

/// Selects a subset of UTXOs with the specified total target value and removes
/// the selected UTXOs from the available set.
///
/// If there are no UTXOs matching the criteria, returns an empty vector.
///
/// PROPERTY: sum(u.value for u in available_set) ≥ target ⇒ !solution.is_empty()
/// POSTCONDITION: !solution.is_empty() ⇒ sum(u.value for u in solution) ≥ target
/// POSTCONDITION:  solution.is_empty() ⇒ available_utxos did not change.
fn greedy(target: u64, available_utxos: &mut BTreeSet<Utxo>) -> Vec<Utxo> {
    let mut solution = vec![];
    let mut goal = target;
    while goal > 0 {
        let utxo = match available_utxos.iter().max_by_key(|u| u.value) {
            Some(max_utxo) if max_utxo.value < goal => max_utxo.clone(),
            Some(_) => available_utxos
                .iter()
                .filter(|u| u.value >= goal)
                .min_by_key(|u| u.value)
                .cloned()
                .expect("bug: there must be at least one UTXO matching the criteria"),
            None => {
                // Not enough available UTXOs to satisfy the request.
                for u in solution {
                    available_utxos.insert(u);
                }
                return vec![];
            }
        };
        goal = goal.saturating_sub(utxo.value);
        assert!(available_utxos.remove(&utxo));
        solution.push(utxo);
    }

    debug_assert!(solution.is_empty() || solution.iter().map(|u| u.value).sum::<u64>() >= target);

    solution
}

/// Gathers ECDSA signatures for all the inputs in the specified unsigned
/// transaction.
///
/// # Panics
///
/// This function panics if the `output_account` map does not have an entry for
/// at least one of the transaction previous output points.
pub async fn sign_transaction(
    key_name: String,
    ecdsa_public_key: &ECDSAPublicKey,
    output_account: &BTreeMap<types::OutPoint, Account>,
    unsigned_tx: tx::UnsignedTransaction,
) -> Result<tx::SignedTransaction, management::CallError> {
    use address::derivation_path;

    let mut signed_inputs = Vec::with_capacity(unsigned_tx.inputs.len());
    let sighasher = tx::TxSigHasher::new(&unsigned_tx);
    for input in &unsigned_tx.inputs {
        let outpoint = &input.previous_output;

        let account = output_account
            .get(outpoint)
            .unwrap_or_else(|| panic!("bug: no account for outpoint {:?}", outpoint));

        let path = derivation_path(account);
        let pubkey = ByteBuf::from(ecdsa_public_key);
        let pkhash = tx::hash160(&pubkey);

        let sighash = sighasher.sighash(input, &pkhash);

        let sec1_signature =
            management::sign_with_ecdsa(key_name.clone(), DerivationPath::new(path), sighash)
                .await?;

        signed_inputs.push(tx::SignedInput {
            signature: signature::EncodedSignature::from_sec1(&sec1_signature),
            pubkey,
            previous_output: outpoint.clone(),
            sequence: input.sequence,
        });
    }
    Ok(tx::SignedTransaction {
        inputs: signed_inputs,
        outputs: unsigned_tx.outputs,
        lock_time: unsigned_tx.lock_time,
    })
}

pub fn fake_sign(unsigned_tx: &tx::UnsignedTransaction) -> tx::SignedTransaction {
    tx::SignedTransaction {
        inputs: unsigned_tx
            .inputs
            .iter()
            .map(|unsigned_input| tx::SignedInput {
                previous_output: unsigned_input.previous_output.clone(),
                sequence: unsigned_input.sequence,
                signature: signature::EncodedSignature::fake(),
                pubkey: ByteBuf::from(vec![0u8; tx::PUBKEY_LEN]),
            })
            .collect(),
        outputs: unsigned_tx.outputs.clone(),
        lock_time: unsigned_tx.lock_time,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildTxError {
    /// The minter does not have enough UTXOs to make the transfer
    /// Try again later after pending transactions have settled.
    NotEnoughFunds,
    /// The withdrawal amount is too low to pay the transfer fee.
    AmountTooLow,
    /// Withdrawal amount of at least one request is too low to cover its share
    /// of the fees. Similar to `AmountTooLow`, but applies to a single
    /// request in a batch.
    DustOutput {
        address: BitcoinAddress,
        amount: u64,
    },
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
    minter_utxos: &mut BTreeSet<Utxo>,
    outputs: Vec<(BitcoinAddress, Satoshi)>,
    main_address: BitcoinAddress,
    fee_per_vbyte: u64,
) -> Result<(tx::UnsignedTransaction, Vec<Utxo>), BuildTxError> {
    assert!(!outputs.is_empty());

    /// Having a sequence number lower than (0xffffffff - 1) signals the use of replacement by fee.
    /// It allows us to increase the fee of a transaction already sent to the mempool.
    /// The rbf option is used in `resubmit_retrieve_btc`.
    /// https://github.com/bitcoin/bips/blob/master/bip-0125.mediawiki
    const SEQUENCE_RBF_ENABLED: u32 = 0xfffffffd;

    let amount = outputs.iter().map(|(_, amount)| amount).sum::<u64>();

    let input_utxos = greedy(amount, minter_utxos);

    if input_utxos.is_empty() {
        return Err(BuildTxError::NotEnoughFunds);
    }

    let inputs_value = input_utxos.iter().map(|u| u.value).sum::<u64>();

    let tx_outputs: Vec<tx::TxOut> = outputs
        .iter()
        .map(|(address, value)| tx::TxOut {
            address: address.clone(),
            value: *value,
        })
        .collect();

    let mut unsigned_tx = tx::UnsignedTransaction {
        inputs: input_utxos
            .iter()
            .map(|utxo| tx::UnsignedInput {
                previous_output: utxo.outpoint.clone(),
                value: utxo.value,
                sequence: SEQUENCE_RBF_ENABLED,
            })
            .collect(),
        outputs: tx_outputs,
        lock_time: 0,
    };

    let tx_vsize = fake_sign(&unsigned_tx).vsize();
    let fee = (tx_vsize as u64 * fee_per_vbyte) / 1000;

    if fee > amount {
        return Err(BuildTxError::AmountTooLow);
    }

    // The default dustRelayFee is 3 sat/vB,
    // which translates to a dust threshold of 546 satoshi for P2PKH outputs.
    // The threshold for other types is lower,
    // so we simply use 546 satoshi as the minimum amount per output.
    const MIN_OUTPUT_AMOUNT: u64 = 546;

    for (output, fee_share) in unsigned_tx.outputs.iter_mut() {
        if output.address != main_address {
            if output.value <= *fee_share + MIN_OUTPUT_AMOUNT {
                return Err(BuildTxError::DustOutput {
                    address: output.address.clone(),
                    amount: output.value,
                });
            }
            output.value = output.value.saturating_sub(*fee_share);
        }
    }

    debug_assert_eq!(
        inputs_value,
        fee + unsigned_tx.outputs.iter().map(|u| u.value).sum::<u64>()
    );

    Ok((unsigned_tx, tx_outputs))
}

/// Computes an estimate for the size of transaction (in vbytes) with the given number of inputs and outputs.
pub fn tx_vsize_estimate(input_count: u64, output_count: u64) -> u64 {
    // See
    // https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki
    // for the transaction structure and
    // https://bitcoin.stackexchange.com/questions/92587/calculate-transaction-fee-for-external-addresses-which-doesnt-belong-to-my-loca/92600#92600
    // for transaction size estimate.
    const INPUT_SIZE_VBYTES: u64 = 68;
    const OUTPUT_SIZE_VBYTES: u64 = 31;
    const TX_OVERHEAD_VBYTES: u64 = 11;

    input_count * INPUT_SIZE_VBYTES + output_count * OUTPUT_SIZE_VBYTES + TX_OVERHEAD_VBYTES
}

/// Computes an estimate for the retrieve_btc fee.
///
/// Arguments:
///   * `available_utxos` - the list of UTXOs available to the minter.
///   * `maybe_amount` - the withdrawal amount.
///   * `median_fee_millisatoshi_per_vbyte` - the median network fee, in millisatoshi per vbyte.
pub fn estimate_fee(
    available_utxos: &BTreeSet<Utxo>,
    maybe_amount: Option<u64>,
    median_fee_millisatoshi_per_vbyte: u64,
    kyt_fee: u64,
) -> WithdrawalFee {
    const DEFAULT_INPUT_COUNT: u64 = 3;
    // One output for the caller and one for the change.
    const DEFAULT_OUTPUT_COUNT: u64 = 2;
    let input_count = match maybe_amount {
        Some(amount) => {
            // We simulate the algorithm that selects UTXOs for the
            // specified amount. If the withdrawal rate is low, we
            // should get the exact number of inputs that the minter
            // will use.
            let mut utxos = available_utxos.clone();
            let selected_utxos = greedy(amount, &mut utxos);

            if !selected_utxos.is_empty() {
                selected_utxos.len() as u64
            } else {
                DEFAULT_INPUT_COUNT
            }
        }
        None => DEFAULT_INPUT_COUNT,
    };

    let vsize = tx_vsize_estimate(input_count, DEFAULT_OUTPUT_COUNT);
    let minter_fee = MINTER_FEE_PER_INPUT * input_count
        + MINTER_FEE_PER_OUTPUT * DEFAULT_OUTPUT_COUNT
        + MINTER_FEE_CONSTANT;
    // We subtract one from the outputs because the minter's output
    // does not participate in fees distribution.
    let bitcoin_fee =
        vsize * median_fee_millisatoshi_per_vbyte / 1000 / (DEFAULT_OUTPUT_COUNT - 1).max(1);
    let minter_fee = minter_fee / (DEFAULT_OUTPUT_COUNT - 1).max(1);
    WithdrawalFee {
        minter_fee: kyt_fee + minter_fee,
        bitcoin_fee,
    }
}

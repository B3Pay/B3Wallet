pub mod address;
pub mod api;
pub mod btc;
pub mod error;
pub mod network;
pub mod signature;
pub mod tx;
pub mod types;
pub mod utils;
pub mod utxos;

#[cfg(test)]
pub mod test;

use address::BitcoinAddress;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::bitcoin::MillisatoshiPerByte;
use serde::Serialize;
use std::time::Duration;
use types::Utxo;

use self::network::BitcoinNetwork;

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
    network: BitcoinNetwork,
    ecdsa_public_key: ECDSAPublicKey,
    unsigned_tx: tx::UnsignedTransaction,
    /// The original requests that we keep around to place back to the queue
    /// if the signature fails.
    requests: Vec<RetrieveBtcRequest>,
    /// The list of UTXOs we use as transaction inputs.
    utxos: Vec<Utxo>,
}

// A pending retrieve btc request
#[derive(candid::CandidType, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrieveBtcRequest {
    /// The amount to convert to BTC.
    /// The minter withdraws BTC transfer fees from this amount.
    pub amount: u64,
    /// The destination BTC address.
    pub address: BitcoinAddress,
    /// The BURN transaction index on the ledger.
    /// Serves as a unique request identifier.
    pub block_index: u64,
    /// The time at which the minter accepted the request.
    pub received_at: u64,
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

fn finalization_time_estimate(min_confirmations: u32, network: BitcoinNetwork) -> Duration {
    Duration::from_nanos(
        min_confirmations as u64
            * match network {
                BitcoinNetwork::Mainnet => 10 * MIN_NANOS,
                BitcoinNetwork::Testnet => MIN_NANOS,
                BitcoinNetwork::Regtest => SEC_NANOS,
            },
    )
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

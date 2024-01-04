use candid::{CandidType, Deserialize};

pub type BtcTxId = String;

pub type BtcTxHash = [u8; 32];

pub type Txid = [u8; 32];

pub type Satoshi = u64;

/// A reference to a transaction output.
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OutPoint {
    /// A cryptographic hash of the transaction.
    /// A transaction can output multiple UTXOs.
    #[serde(with = "serde_bytes")]
    pub txid: Vec<u8>,
    /// The index of the output within the transaction.
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: Vec<u8>, vout: u32) -> Self {
        Self { txid, vout }
    }
}

/// An unspent transaction output.
#[derive(CandidType, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: Satoshi,
    pub height: u32,
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum UtxoStatus {
    ValueTooSmall(Utxo),
    Tainted(Utxo),
    Checked(Utxo),
    Minted {
        block_index: u64,
        minted_amount: u64,
        utxo: Utxo,
    },
}

/// A transaction output storing the minter's change.
#[derive(candid::CandidType, Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ChangeOutput {
    /// The index of the output in the transaction.
    pub vout: u32,
    /// The value of the output.
    pub value: u64,
}

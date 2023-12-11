use std::fmt;
use std::str::FromStr;

use candid::{CandidType, Deserialize};

pub type BtcTxId = String;

pub type BtcTxHash = [u8; 32];

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

/// An unspent transaction output.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone, Hash, Eq)]
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

#[derive(CandidType, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Txid([u8; 32]);

impl AsRef<[u8]> for Txid {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Txid> for [u8; 32] {
    fn from(txid: Txid) -> Self {
        txid.0
    }
}

impl serde::Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> serde::de::Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct TxidVisitor;

        impl<'de> serde::de::Visitor<'de> for TxidVisitor {
            type Value = Txid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a 32-byte array")
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match TryInto::<[u8; 32]>::try_into(value) {
                    Ok(txid) => Ok(Txid(txid)),
                    Err(_) => Err(E::invalid_length(value.len(), &self)),
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error;
                if let Some(size_hint) = seq.size_hint() {
                    if size_hint != 32 {
                        return Err(A::Error::invalid_length(size_hint, &self));
                    }
                }
                let mut bytes = [0u8; 32];
                let mut i = 0;
                while let Some(byte) = seq.next_element()? {
                    if i == 32 {
                        return Err(A::Error::invalid_length(i + 1, &self));
                    }

                    bytes[i] = byte;
                    i += 1;
                }
                if i != 32 {
                    return Err(A::Error::invalid_length(i, &self));
                }
                Ok(Txid(bytes))
            }
        }

        deserializer.deserialize_bytes(TxidVisitor)
    }
}

impl fmt::Display for Txid {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // In Bitcoin, you display hash bytes in reverse order.
        //
        // > Due to historical accident, the tx and block hashes that bitcoin core
        // > uses are byte-reversed. I’m not entirely sure why. Maybe something
        // > like using openssl bignum to store hashes or something like that,
        // > then printing them as a number.
        // > -- Wladimir van der Laan
        //
        // Source: https://learnmeabitcoin.com/technical/txid
        for b in self.0.iter().rev() {
            write!(fmt, "{:02x}", *b)?
        }
        Ok(())
    }
}

impl From<[u8; 32]> for Txid {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl TryFrom<&'_ [u8]> for Txid {
    type Error = core::array::TryFromSliceError;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let txid: [u8; 32] = bytes.try_into()?;
        Ok(Txid(txid))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxidFromStrError {
    InvalidChar(u8),
    InvalidLength { expected: usize, actual: usize },
}

impl fmt::Display for TxidFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "char {c} is not a valid hex"),
            Self::InvalidLength { expected, actual } => write!(
                f,
                "Bitcoin transaction id must be precisely {expected} characters, got {actual}"
            ),
        }
    }
}

impl FromStr for Txid {
    type Err = TxidFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn decode_hex_char(c: u8) -> Result<u8, TxidFromStrError> {
            match c {
                b'A'..=b'F' => Ok(c - b'A' + 10),
                b'a'..=b'f' => Ok(c - b'a' + 10),
                b'0'..=b'9' => Ok(c - b'0'),
                _ => Err(TxidFromStrError::InvalidChar(c)),
            }
        }
        if s.len() != 64 {
            return Err(TxidFromStrError::InvalidLength {
                expected: 64,
                actual: s.len(),
            });
        }
        let mut bytes = [0u8; 32];
        let chars = s.as_bytes();
        for i in 0..32 {
            bytes[31 - i] =
                (decode_hex_char(chars[2 * i])? << 4) | decode_hex_char(chars[2 * i + 1])?;
        }
        Ok(Self(bytes))
    }
}

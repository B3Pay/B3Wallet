use super::Nonce;
use std::fmt;
use std::ops::{Add, Sub};

impl Add for Nonce {
    type Output = Nonce;

    fn add(self, other: Nonce) -> Nonce {
        Nonce(self.0 + other.0)
    }
}

impl Sub for Nonce {
    type Output = Nonce;

    fn sub(self, other: Nonce) -> Nonce {
        Nonce(self.0.saturating_sub(other.0))
    }
}

impl TryFrom<&[u8]> for Nonce {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(u64::from_le_bytes(value.try_into()?)))
    }
}

impl From<u64> for Nonce {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Nonce> for u64 {
    fn from(value: Nonce) -> Self {
        value.0
    }
}

impl From<Nonce> for Vec<u8> {
    fn from(value: Nonce) -> Self {
        value.0.to_le_bytes().to_vec()
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "stable_memory")]
use ic_stable_structures::storable::Bound;

#[cfg(feature = "stable_memory")]
impl ic_stable_structures::Storable for Nonce {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.to_le_bytes().to_vec().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Nonce::from_le_bytes(bytes[0..8].try_into().unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: true,
        max_size: 8,
    };
}

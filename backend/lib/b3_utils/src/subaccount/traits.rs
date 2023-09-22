use std::{cmp, fmt, hash, mem::size_of, str::FromStr};

use candid::Principal;

use super::{Subaccount, SubaccountError};

impl Eq for Subaccount {}

impl cmp::PartialOrd for Subaccount {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Subaccount {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl hash::Hash for Subaccount {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<Principal> for Subaccount {
    fn from(principal: Principal) -> Self {
        let mut subaccount = [0; size_of::<Subaccount>()];
        let principal_id = principal.as_slice();

        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

        Subaccount(subaccount)
    }
}

impl From<[u8; 32]> for Subaccount {
    fn from(bytes: [u8; 32]) -> Self {
        Subaccount(bytes)
    }
}

impl TryFrom<Vec<u8>> for Subaccount {
    type Error = SubaccountError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(SubaccountError::LengthError(value.len()));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&value);

        Ok(Self(bytes))
    }
}

impl FromStr for Subaccount {
    type Err = SubaccountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_hex(s)?)
    }
}

impl fmt::Display for Subaccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        hex::encode(&self.0).fmt(f)
    }
}

#[cfg(feature = "stable_memory")]
use ic_stable_structures::storable::Bound;

#[cfg(feature = "stable_memory")]
impl ic_stable_structures::Storable for Subaccount {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Subaccount::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: true,
        max_size: 32,
    };
}

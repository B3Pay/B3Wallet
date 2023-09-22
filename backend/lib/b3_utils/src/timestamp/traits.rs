use std::fmt;

use super::NanoTimeStamp;

impl From<u64> for NanoTimeStamp {
    fn from(nanos: u64) -> Self {
        NanoTimeStamp(nanos)
    }
}

impl From<NanoTimeStamp> for u64 {
    fn from(ts: NanoTimeStamp) -> Self {
        ts.0
    }
}

impl From<NanoTimeStamp> for i64 {
    fn from(ts: NanoTimeStamp) -> Self {
        ts.0 as i64
    }
}

impl fmt::Display for NanoTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0 / Self::NS_PER_SECOND;
        let nanos = self.0 % Self::NS_PER_SECOND;
        write!(f, "{}.{:09}", secs, nanos)
    }
}

#[cfg(feature = "stable_memory")]
use ic_stable_structures::storable::Bound;

#[cfg(feature = "stable_memory")]
impl ic_stable_structures::Storable for NanoTimeStamp {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.to_le_bytes().to_vec().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        NanoTimeStamp::from_le_bytes(bytes[0..8].try_into().unwrap())
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: true,
        max_size: 8,
    };
}

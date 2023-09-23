use super::Wasm;
use serde_bytes::ByteBuf;

#[cfg(feature = "stable_memory")]
use crate::memory::types::{Bound, Storable};

#[cfg(feature = "stable_memory")]
impl Storable for Wasm {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(ByteBuf::from(bytes))
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_vec().into()
    }
}

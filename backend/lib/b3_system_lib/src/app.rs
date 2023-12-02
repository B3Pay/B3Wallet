use b3_utils::memory::types::{Bound, Storable};
use candid::CandidType;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::types::ReleaseVersion;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct App {
    pub id: String,
    pub name: String,
    pub description: String,
    pub release: ReleaseVersion,
}

impl App {
    pub fn new(id: String, name: String, description: String, release: ReleaseVersion) -> Self {
        App {
            id,
            name,
            description,
            release,
        }
    }
}

impl Storable for App {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        into_writer(&self, &mut bytes).unwrap();
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_reader(&mut Cursor::new(&bytes)).unwrap()
    }
}

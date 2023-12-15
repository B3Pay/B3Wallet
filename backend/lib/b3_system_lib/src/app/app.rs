use b3_utils::{
    ledger::Metadata,
    memory::types::{Bound, Storable},
    NanoTimeStamp,
};
use candid::{CandidType, Principal};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::types::ReleaseVersion;

use super::{release::Release, store::with_releases};

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct App {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_by: Principal,
    pub created_at: NanoTimeStamp,
    pub updated_at: NanoTimeStamp,
    pub releases: Vec<ReleaseVersion>,
    pub metadata: Metadata,
}

impl App {
    pub fn new(id: String, name: String, description: String) -> Self {
        App {
            id,
            name,
            description,
            created_at: NanoTimeStamp::now(),
            updated_at: NanoTimeStamp::now(),
            created_by: Principal::anonymous(),
            releases: Vec::new(),
            metadata: Metadata::new(),
        }
    }

    pub fn add_release(&mut self, version: ReleaseVersion) {
        self.updated_at = NanoTimeStamp::now();
        self.releases.push(version);
    }

    pub fn remove_release(&mut self, version: ReleaseVersion) {
        self.updated_at = NanoTimeStamp::now();
        self.releases.retain(|v| v != &version);
    }

    pub fn update_release(&mut self, version: ReleaseVersion, new_version: ReleaseVersion) {
        self.updated_at = NanoTimeStamp::now();
        self.releases.retain(|v| v != &version);
        self.releases.push(new_version);
    }

    pub fn get_release(&self, version: &ReleaseVersion) -> Option<Release> {
        with_releases(|releases| releases.get(version))
    }

    pub fn get_latest_release(&self) -> Option<Release> {
        let latest_version = self.releases.iter().max().unwrap();

        self.get_release(latest_version)
    }

    pub fn get_releases(&self) -> Vec<Release> {
        with_releases(|releases| {
            self.releases
                .iter()
                .filter_map(|version| releases.get(version))
                .collect()
        })
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

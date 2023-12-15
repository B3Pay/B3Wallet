use b3_utils::{
    ledger::Metadata,
    memory::types::{Bound, Storable},
    name_to_slug,
    nonce::Nonce,
    wasm::WasmHash,
    NanoTimeStamp,
};
use candid::Principal;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(test)]
use b3_utils::mocks::id_mock as ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

use super::{
    error::AppSystemError,
    release::Release,
    store::{with_release_mut, with_releases, with_releases_mut},
    types::{AppId, AppView, CreateAppArgs, CreateReleaseArgs, ReleaseView, ReleaseViews},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct App {
    id: AppId,
    name: String,
    description: String,
    created_by: Principal,
    created_at: NanoTimeStamp,
    updated_at: NanoTimeStamp,
    release_hashes: Vec<WasmHash>,
    metadata: Metadata,
    install_count: Nonce,
}

// Create the App struct
impl App {
    pub fn new(app_args: CreateAppArgs) -> Self {
        let CreateAppArgs {
            name,
            description,
            metadata,
        } = app_args;

        let created_by = ic_cdk_id();
        let id = name_to_slug(&name);

        Self {
            id,
            name,
            metadata,
            created_by,
            description,
            release_hashes: Vec::new(),
            created_at: NanoTimeStamp::now(),
            updated_at: NanoTimeStamp::now(),
            install_count: Nonce::zero(),
        }
    }
}

// Write to the App struct
impl App {
    pub fn update(&mut self, app_args: CreateAppArgs) -> Self {
        let CreateAppArgs {
            name,
            description,
            metadata,
        } = app_args;

        self.name = name;
        self.metadata = metadata;
        self.description = description;
        self.updated_at = NanoTimeStamp::now();

        self.clone()
    }

    fn add_release_hash(&mut self, wasm_hash: WasmHash) {
        self.updated_at = NanoTimeStamp::now();
        self.release_hashes.push(wasm_hash);
    }

    pub fn add_release(&mut self, release_args: CreateReleaseArgs) {
        if let Some(_) = self.release(&release_args.wasm_hash) {
            return;
        }

        let wasm_hash = release_args.wasm_hash.clone();

        self.add_release_hash(wasm_hash);

        let release = Release::new(release_args);

        with_releases_mut(|releases| releases.insert(wasm_hash, release));
    }

    pub fn deprecate_release(&mut self, wasm_hash: WasmHash) -> Result<Release, AppSystemError> {
        with_release_mut(&wasm_hash, |release| {
            release.deprecate();

            release.clone()
        })
    }
}

// Read from the App struct
impl App {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn view(&self) -> AppView {
        AppView {
            id: self.id(),
            name: self.name.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            description: self.description.clone(),
            created_by: self.created_by.to_string(),
            install_count: self.install_count.clone(),
            latest_release: self.latest_release_view(),
        }
    }

    pub fn release_hash(&self, wasm_hash: &WasmHash) -> Option<WasmHash> {
        self.release_hashes
            .iter()
            .find(|v| v == &wasm_hash)
            .cloned()
    }

    pub fn release_hashes(&self) -> Vec<WasmHash> {
        self.release_hashes.clone()
    }

    // DIRECT ACCESS TO RELEASES
    pub fn release(&self, wasm_hash: &WasmHash) -> Option<Release> {
        with_releases(|releases| releases.get(wasm_hash))
    }

    pub fn release_view(&self, wasm_hash: &WasmHash) -> Option<ReleaseView> {
        self.release(wasm_hash).map(|release| release.view())
    }

    pub fn release_views(&self) -> ReleaseViews {
        self.releases()
            .iter()
            .map(|release| release.view())
            .collect()
    }

    pub fn latest_release(&self) -> Option<Release> {
        let latest_hash = self.release_hashes.iter().max().cloned()?;

        self.release(&latest_hash)
    }

    pub fn latest_release_view(&self) -> Option<ReleaseView> {
        let latest_hash = self.release_hashes.iter().max().cloned()?;

        self.release_view(&latest_hash)
    }

    pub fn releases(&self) -> Vec<Release> {
        with_releases(|releases| {
            self.release_hashes
                .iter()
                .filter_map(|wasm_hash| releases.get(wasm_hash))
                .collect()
        })
    }

    pub fn releases_view(&self) -> Vec<super::types::ReleaseView> {
        self.releases()
            .iter()
            .map(|release| release.view())
            .collect()
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

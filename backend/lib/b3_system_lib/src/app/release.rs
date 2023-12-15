use b3_utils::{
    api::AppVersion,
    memory::types::{Bound, Storable},
    wasm::{with_wasm_mut_cache, Wasm, WasmHash, WasmSize},
    NanoTimeStamp,
};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use super::{error::AppSystemError, types::AppReleaseArgs};

use super::store::{with_release_wasm, with_wasm_map_mut};

#[derive(Deserialize, Serialize, Clone)]
pub struct Release {
    name: String,
    date: NanoTimeStamp,
    size: WasmSize,
    version: AppVersion,
    deprecated: bool,
    features: String,
}

// Create the Release struct
impl Release {
    pub fn new(release_args: AppReleaseArgs) -> Self {
        Self {
            deprecated: false,
            size: release_args.size,
            name: release_args.name,
            date: NanoTimeStamp::now(),
            version: release_args.version,
            features: release_args.features,
        }
    }
}

// Write to the Release struct
impl Release {
    pub fn load_wasm(&mut self, blob: &Vec<u8>) -> Result<WasmSize, AppSystemError> {
        if self.is_loaded() {
            return Err(AppSystemError::WasmAlreadyLoaded);
        }

        let wasm_len = with_wasm_mut_cache(|wasm| wasm.load(blob));

        if wasm_len >= self.size {
            with_wasm_mut_cache(|wasm| {
                with_wasm_map_mut(|wasm_map| {
                    wasm_map.insert(self.version.clone(), wasm.clone()).unwrap();
                });

                wasm.unload();
            });
        }

        Ok(wasm_len)
    }

    pub fn unload_wasm(&mut self) -> WasmSize {
        with_wasm_map_mut(|wasm_map| {
            wasm_map.remove(&self.version);
        });

        with_wasm_mut_cache(|wasm| wasm.unload())
    }

    pub fn deprecate(&mut self) {
        with_wasm_map_mut(|wasm_map| {
            wasm_map.remove(&self.version);
        });

        self.deprecated = true;
    }

    pub fn edit_feature(&mut self, feature: String) {
        self.features = feature;
    }

    pub fn remove_feature(&mut self) {
        self.features = "".to_string();
    }
}

// Read of the Release struct
impl Release {
    pub fn view(&self) -> super::types::ReleaseView {
        super::types::ReleaseView {
            name: self.name.clone(),
            date: self.date.clone(),
            size: self.size,
            version: self.version.clone(),
            deprecated: self.deprecated,
            features: self.features.clone(),
        }
    }

    pub fn is_loading(&self) -> bool {
        with_release_wasm(&self.version, |wasm| wasm.is_loading(self.size)).unwrap_or(false)
    }

    pub fn is_loaded(&self) -> bool {
        with_release_wasm(&self.version, |wasm| wasm.is_loaded(self.size)).unwrap_or(false)
    }

    pub fn wasm(&self) -> Result<Wasm, AppSystemError> {
        with_release_wasm(&self.version, |wasm| wasm)
    }

    pub fn verify_hash(&self, hash: &WasmHash) -> bool {
        with_release_wasm(&self.version, |wasm| wasm.verify_hash(hash)).unwrap_or(false)
    }

    pub fn wasm_hash(&self) -> Result<WasmHash, AppSystemError> {
        with_release_wasm(&self.version, |wasm| wasm.hash())
    }

    pub fn wasm_size(&self) -> Result<WasmSize, AppSystemError> {
        with_release_wasm(&self.version, |wasm| wasm.len())
    }

    pub fn is_deprecated(&self) -> bool {
        self.deprecated
    }
}

impl Storable for Release {
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

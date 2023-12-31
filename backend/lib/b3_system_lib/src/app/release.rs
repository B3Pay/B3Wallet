use b3_utils::{
    api::AppVersion,
    memory::types::{Bound, Storable},
    vec_to_hex_string_with_0x,
    wasm::{with_wasm_mut_cache, Wasm, WasmHash, WasmHashString, WasmSize},
    NanoTimeStamp,
};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use ic_cdk::api::management_canister::main::WasmModule;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use super::{
    error::AppSystemError,
    store::{with_wasm, with_wasms_mut},
    types::{AppId, CreateReleaseArgs, ReleaseView},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Release {
    app_id: AppId,
    date: NanoTimeStamp,
    size: WasmSize,
    version: AppVersion,
    deprecated: bool,
    features: String,
    wasm_hash: WasmHash,
}

// Create the Release struct
impl Release {
    pub fn new(release_args: CreateReleaseArgs) -> Self {
        Self {
            app_id: release_args.id,
            deprecated: false,
            size: release_args.size,
            date: NanoTimeStamp::now(),
            version: release_args.version,
            features: release_args.features,
            wasm_hash: release_args.wasm_hash,
        }
    }
}

// Write to the Release struct
impl Release {
    pub fn update(&mut self, release_args: CreateReleaseArgs) -> Self {
        self.size = release_args.size;
        self.version = release_args.version;
        self.features = release_args.features;

        self.clone()
    }

    pub fn load_wasm_chunk(&mut self, blob: &Vec<u8>) -> Result<WasmSize, AppSystemError> {
        if self.is_loaded() {
            return Err(AppSystemError::WasmAlreadyLoaded);
        }

        with_wasm_mut_cache(|wasm| {
            let wasm_len = wasm.load(blob);

            if wasm_len >= self.size {
                with_wasms_mut(|wasm_map| {
                    wasm_map.insert(self.wasm_hash, wasm.clone());
                });

                wasm.unload();
            }

            Ok(wasm_len)
        })
    }

    pub fn unload_wasm(&mut self) -> WasmSize {
        with_wasms_mut(|wasm_map| {
            wasm_map.remove(&self.wasm_hash);
        });

        with_wasm_mut_cache(|wasm| wasm.unload())
    }

    pub fn deprecate(&mut self) {
        with_wasms_mut(|wasm_map| {
            wasm_map.remove(&self.wasm_hash);
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
    pub fn id(&self) -> String {
        self.app_id.clone()
    }

    pub fn view(&self) -> ReleaseView {
        ReleaseView {
            name: self.app_id.clone(),
            date: self.date.clone(),
            size: self.size,
            version: self.version.clone(),
            deprecated: self.deprecated,
            features: self.features.clone(),
            wasm_hash: vec_to_hex_string_with_0x(self.wasm_hash),
        }
    }

    pub fn is_loading(&self) -> bool {
        with_wasm(&self.wasm_hash, |wasm| wasm.is_loading(self.size)).unwrap_or(false)
    }

    pub fn is_loaded(&self) -> bool {
        with_wasm(&self.wasm_hash, |wasm| wasm.is_loaded(self.size)).unwrap_or(false)
    }

    pub fn wasm(&self) -> Result<Wasm, AppSystemError> {
        with_wasm(&self.wasm_hash, |wasm| wasm)
    }

    pub fn wasm_module(&self) -> Result<WasmModule, AppSystemError> {
        with_wasm(&self.wasm_hash, |wasm| wasm.bytes())
    }

    pub fn verify_hash(&self, hash: &WasmHash) -> bool {
        with_wasm(&self.wasm_hash, |wasm| wasm.verify_hash(hash)).unwrap_or(false)
    }

    pub fn wasm_hash(&self) -> Result<WasmHash, AppSystemError> {
        with_wasm(&self.wasm_hash, |wasm| wasm.hash())
    }

    pub fn wasm_hash_string(&self) -> Result<WasmHashString, AppSystemError> {
        with_wasm(&self.wasm_hash, |wasm| wasm.hash_string())
    }

    pub fn wasm_size(&self) -> Result<WasmSize, AppSystemError> {
        with_wasm(&self.wasm_hash, |wasm| wasm.len())
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

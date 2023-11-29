use b3_utils::{
    ledger::types::WalletVersion,
    memory::types::{Bound, Storable},
    wasm::{with_wasm_mut_cache, Wasm, WasmHash, WasmSize},
    NanoTimeStamp,
};
use candid::CandidType;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::{
    error::SystemError,
    store::{with_release_wasm, with_wasm_map_mut},
    types::{Features, ReleaseArgs},
};

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Release {
    pub name: String,
    pub date: NanoTimeStamp,
    pub size: WasmSize,
    pub hash: WasmHash,
    pub version: WalletVersion,
    pub deprecated: bool,
    pub features: Features,
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

impl Release {
    pub fn new(release_args: ReleaseArgs) -> Self {
        let version = release_args.version.clone();

        with_wasm_map_mut(|wasm_map| {
            wasm_map.insert(version, Wasm::default());
        });

        release_args.into()
    }

    pub fn is_loading(&self) -> bool {
        with_release_wasm(&self.version, |wasm| wasm.is_loading(self.size)).unwrap_or(false)
    }

    pub fn is_loaded(&self) -> bool {
        with_release_wasm(&self.version, |wasm| wasm.is_loaded(self.size)).unwrap_or(false)
    }

    pub fn is_same_hash(&self, hash: &WasmHash) -> bool {
        self.hash == *hash
    }

    pub fn wasm(&self) -> Result<Wasm, SystemError> {
        with_release_wasm(&self.version, |wasm| Ok(wasm))?
    }

    pub fn load_wasm(&mut self, blob: &Vec<u8>) -> Result<WasmSize, SystemError> {
        if self.is_loaded() {
            return Err(SystemError::WasmAlreadyLoaded);
        }

        let wasm_len = with_wasm_mut_cache(|wasm| wasm.load(blob));

        if wasm_len >= self.size {
            with_wasm_mut_cache(|wasm| {
                self.hash = wasm.generate_hash();

                ic_cdk::println!("wasm hash: {:?}", self.hash);

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

    pub fn update(&mut self, release: ReleaseArgs) {
        self.size = release.size;
        self.features = release.features;
        self.date = NanoTimeStamp::now();
    }

    pub fn deprecate(&mut self) {
        with_wasm_map_mut(|wasm_map| {
            wasm_map.remove(&self.version);
        });

        self.deprecated = true;
    }

    pub fn add_feature(&mut self, feature: String) {
        self.features.push(feature);
    }

    pub fn remove_feature(&mut self, feature: String) {
        self.features.retain(|f| f != &feature);
    }
}

impl Default for Release {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            version: "0.0.0".to_string(),
            date: NanoTimeStamp(0),
            size: 0,
            hash: WasmHash::default(),
            deprecated: false,
            features: vec!["".to_string()],
        }
    }
}

impl From<ReleaseArgs> for Release {
    fn from(args: ReleaseArgs) -> Self {
        Self {
            name: args.name,
            date: NanoTimeStamp::now(),
            size: args.size,
            deprecated: false,
            hash: WasmHash::default(),
            version: args.version,
            features: args.features,
        }
    }
}

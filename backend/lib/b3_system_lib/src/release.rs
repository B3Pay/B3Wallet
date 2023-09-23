use b3_utils::{
    wasm::{with_wasm, with_wasm_mut, Wasm, WasmHash, WasmModule, WasmSize},
    NanoTimeStamp,
};

pub mod names;

use crate::{
    error::SystemError,
    store::{with_release_wasm, with_wasm_map_mut},
    types::{Release, ReleaseArgs},
};

impl Default for Release {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            version: "0.0.0".to_string(),
            date: NanoTimeStamp(0),
            size: 0,
            hash: WasmHash::default(),
            deprecated: false,
            features: None,
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

    pub fn wasm(&self) -> Result<WasmModule, SystemError> {
        let wasm = with_release_wasm(&self.version, |wasm| wasm.0.clone())?;

        Ok(wasm.to_vec())
    }

    pub fn load_wasm(&mut self, blob: &Vec<u8>) -> Result<WasmSize, SystemError> {
        if self.is_loaded() {
            return Err(SystemError::WasmAlreadyLoaded);
        }

        let wasm_len = with_wasm_mut(|wasm| wasm.load(blob));

        if wasm_len >= self.size {
            with_wasm_map_mut(|wasm_map| {
                let wasm = with_wasm(|wasm| wasm.clone());

                self.hash = wasm.generate_hash();

                wasm_map.insert(self.version.clone(), wasm).unwrap();
            });
        }

        Ok(wasm_len)
    }

    pub fn unload_wasm(&mut self) -> WasmSize {
        with_wasm_map_mut(|wasm_map| {
            wasm_map.remove(&self.version);
        });

        with_wasm_mut(|wasm| wasm.unload())
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
        match &mut self.features {
            Some(features) => features.push(feature),
            None => self.features = Some(vec![feature]),
        }
    }

    pub fn remove_feature(&mut self, feature: &str) {
        match &mut self.features {
            Some(features) => {
                let index = features.iter().position(|f| f == feature);

                if let Some(index) = index {
                    features.remove(index);
                }
            }
            None => {}
        }
    }
}

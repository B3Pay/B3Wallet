mod query;
mod update;
mod wasm;
use ic_cdk::api::time;

use crate::{
    store::{with_wasm, with_wasm_map_mut, with_wasm_mut},
    types::{Features, Release, ReleaseArgs, Version, Wasm},
};

impl Default for Release {
    fn default() -> Self {
        Self {
            version: "0.0.0".to_string(),
            date: 0,
            size: 0,
            hash: String::new(),
            deprecated: false,
            features: None,
        }
    }
}

impl From<ReleaseArgs> for Release {
    fn from(args: ReleaseArgs) -> Self {
        Self {
            date: time(),
            size: args.size,
            deprecated: false,
            hash: String::new(),
            version: args.version,
            features: args.features,
        }
    }
}

impl Release {
    pub fn new(release: ReleaseArgs) -> Self {
        let version = release.version.clone();

        with_wasm_map_mut(|wasm_map| {
            wasm_map.insert(version, Wasm::default());
        });

        release.into()
    }

    pub fn load_wasm(&mut self, blob: &Vec<u8>) -> Result<usize, String> {
        if self.is_loaded() {
            return Err("Release is already loaded!".to_string());
        }

        let wasm_len = with_wasm_mut(&self.version, |wasm| wasm.load(blob))?;

        if wasm_len >= self.size {
            let wasm_hash = with_wasm(&self.version, |wasm| wasm.generate_hash())?;

            self.hash = wasm_hash;
        }

        Ok(wasm_len)
    }

    pub fn unload_wasm(&mut self) {
        with_wasm_mut(&self.version, |wasm| wasm.clear()).unwrap_or_else(|_| ());
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

    pub fn is_empty(&self) -> bool {
        with_wasm(&self.version, |wasm| wasm.is_empty()).unwrap_or_else(|_| true)
    }

    pub fn is_loading(&self) -> bool {
        with_wasm(&self.version, |wasm| wasm.is_loading(self.size)).unwrap_or_else(|_| false)
    }

    pub fn is_loaded(&self) -> bool {
        with_wasm(&self.version, |wasm| wasm.is_loaded(self.size)).unwrap_or_else(|_| false)
    }

    pub fn update(&mut self, release: ReleaseArgs) {
        self.size = release.size;
        self.features = release.features;
        self.date = time();
    }

    pub fn get_wasm(&self) -> Option<Wasm> {
        with_wasm(&self.version, |wasm| wasm.clone()).ok()
    }

    pub fn deprecate(&mut self) {
        with_wasm_map_mut(|wasm_map| {
            wasm_map.remove(&self.version);
        });

        self.deprecated = true;
    }
}

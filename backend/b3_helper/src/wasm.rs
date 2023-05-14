use serde_bytes::ByteBuf;

use crate::{
    sha2_sha256_wasm_hash, sha2_sha256_wasm_hash_string,
    types::{Blob, Wasm, WasmHash, WasmSize},
};

impl Default for Wasm {
    fn default() -> Self {
        Self(ByteBuf::new())
    }
}

impl Wasm {
    pub fn load(&mut self, blob: &Blob) -> WasmSize {
        self.0.extend(blob);

        self.0.len()
    }

    pub fn unload(&mut self) -> WasmSize {
        self.0.clear();

        self.0.len()
    }

    pub fn len(&self) -> WasmSize {
        self.0.len()
    }

    pub fn get(&self) -> Blob {
        self.0.to_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn is_loading(&self, size: usize) -> bool {
        self.0.is_empty() || self.0.len() < size
    }

    pub fn is_loaded(&self, size: usize) -> bool {
        self.0.len() == size
    }

    pub fn generate_hash(&self) -> WasmHash {
        if self.0.is_empty() {
            return WasmHash::default();
        }

        sha2_sha256_wasm_hash(&self.0)
    }

    pub fn generate_hash_string(&self) -> String {
        if self.0.is_empty() {
            return String::default();
        }

        sha2_sha256_wasm_hash_string(&self.0)
    }

    pub fn is_hashed(&self, hash: &WasmHash) -> bool {
        self.generate_hash() == *hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut wasm = Wasm::default();
        let blob = vec![1, 2, 3];

        assert_eq!(wasm.load(&blob), 3);
        assert_eq!(wasm.get(), vec![1, 2, 3]);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());
    }

    #[test]
    fn test_load_multiple() {
        let mut wasm = Wasm::default();
        let blob = vec![1, 2, 3];
        let blob2 = vec![4, 5, 6];

        assert_eq!(wasm.load(&blob), 3);
        assert_eq!(wasm.load(&blob2), 6);
        assert_eq!(wasm.get(), vec![1, 2, 3, 4, 5, 6]);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());

        if wasm.is_loaded(6) {
            wasm.unload();
        }

        assert_eq!(wasm.len(), 0);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());
    }

    #[test]
    fn test_is_empty() {
        let wasm = Wasm::default();

        assert_eq!(wasm.is_empty(), true);
    }

    #[test]
    fn test_is_loading() {
        let mut wasm = Wasm::default();
        let blob = vec![1, 2, 3];

        assert_eq!(wasm.is_loading(3), true);
        assert_eq!(wasm.load(&blob), 3);
        assert_eq!(wasm.is_loading(3), false);
    }

    #[test]
    fn test_is_loaded() {
        let mut wasm = Wasm::default();
        let blob = vec![1, 2, 3];

        assert_eq!(wasm.is_loaded(3), false);
        assert_eq!(wasm.load(&blob), 3);
        assert_eq!(wasm.is_loaded(3), true);
    }

    #[test]
    fn test_generate_hash() {
        let mut wasm = Wasm::default();
        let blob = vec![1, 2, 3];

        assert_eq!(wasm.generate_hash(), WasmHash::default());
        assert_eq!(wasm.load(&blob), 3);

        let actual = wasm.generate_hash();

        let expected = WasmHash::from([
            3, 144, 88, 198, 242, 192, 203, 73, 44, 83, 59, 10, 77, 20, 239, 119, 204, 15, 120,
            171, 204, 206, 213, 40, 125, 132, 161, 162, 1, 28, 251, 129,
        ]);

        assert_eq!(actual, expected);
    }
}

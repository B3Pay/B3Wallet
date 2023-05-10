use sha2::{Digest, Sha256};

use crate::types::{Blob, Wasm, WasmHash};

impl Default for Wasm {
    fn default() -> Self {
        Self(Blob::new())
    }
}

impl Wasm {
    pub fn load(&mut self, blob: &Blob) -> usize {
        self.extend(blob);

        self.len()
    }

    pub fn get(&self) -> Blob {
        self.0.clone()
    }

    pub fn extend(&mut self, blob: &Blob) {
        self.0.extend(blob);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_loading(&self, size: usize) -> bool {
        self.is_empty() || self.len() < size
    }

    pub fn is_loaded(&self, size: usize) -> bool {
        self.len() == size
    }

    pub fn generate_hash(&self) -> WasmHash {
        if self.is_empty() {
            return WasmHash::default();
        }

        let mut hasher = Sha256::new();
        hasher.update(&self.0);

        hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
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
        assert_eq!(wasm.0, vec![1, 2, 3]);

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
        assert_eq!(wasm.0, vec![1, 2, 3, 4, 5, 6]);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());

        if wasm.is_loaded(6) {
            wasm.clear();
        }

        assert_eq!(wasm.len(), 0);

        println!("{:?}", wasm.get());
        println!("{:?}", wasm.generate_hash());
    }
}

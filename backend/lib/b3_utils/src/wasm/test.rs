#[cfg(test)]
mod tests {
    use crate::wasm::{types::WasmHash, Wasm};

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
